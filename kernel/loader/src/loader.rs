use core::ptr;

use cortex_a::{
    asm::barrier,
    registers::{MAIR_EL1, SCTLR_EL1, TCR_EL1, TTBR0_EL1, TTBR1_EL1},
};
use tock_registers::{interfaces::Writeable, registers::InMemoryRegister};

use crate::{
    bsp,
    paging::{PageTableMapper, PhysAddr, VirtAddr, PAGE_DESCRIPTOR},
    rt, INITAL_PAGE_ALLOCATOR,
};

/// Address mappings of all relevant kernel segments in physical memory.
///
/// This is passed to [`main`] in order to relocate and randomize all the kernel mappings
/// in memory after enabling KASLR.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct KernelMap {
    /// The start offset of the kernel's `.text` segment.
    text_start: u32,
    /// The end offset of the kernel's `.text` segment.
    text_end: u32,
    /// The start offset of the kernel's `.rodata` segment.
    rodata_start: u32,
    /// The end offset of the kernel's `.rodata` segment.
    rodata_end: u32,
    /// The start offset of the kernel's `.data` segment.
    data_start: u32,
    /// The end offset of the kernel's `.data` segment.
    data_end: u32,
    /// The start offset of the kernel's `.bss` segment.
    bss_start: u32,
    /// The end offset of the kernel's `.bss` segment.
    bss_end: u32,
    /// The end offset of the kernel's `.ini1` segment.
    ini1: u32,
    /// The start offset of the kernel's `.dynamic` segment.
    dynamic: u32,
    /// The start offset of the kernel's `.init_array` segment.
    init_array_start: u32,
    /// The end offset of the kernel's `.init_array` segment.
    init_array_end: u32,
}

assert_eq_size!(KernelMap, [u8; 0x30]);

pub const INI1_MAGIC: [u8; 4] = *b"INI1";

/// The maximum size of the INI1 section (12 MiB).
pub const MAX_INI1_SIZE: usize = 12 << 20;

/// The amount of bytes that will be reserved for the kernel to use and
/// thus won't be overwritten by the loader.
pub const KERNEL_DATA_SIZE: usize = 0x1728000;

/// Amount of memory that will be reserved *additionally* to the [`KERNEL_DATA_SIZE`] if
/// the kernel requests a larger amount reserved data.
pub const ADDITIONAL_KERNEL_DATA_SIZE: usize = 0x68000;

/// The header of the INI1 process, which is the first process that will be ran by the kernel.
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct InitialProcessBinaryHeader {
    /// Magic number: b"INI1"
    pub magic: [u8; 4],
    /// The size of the process binary
    pub size: u32,
    /// Number of KIPs. Must be lower than 0x51
    pub num_processes: u32,
    /// Reserved field
    pub _reserved: u32,
}

assert_eq_align!(InitialProcessBinaryHeader, u32);

/// Relocates the kernel to a random base address, identity maps the kernel and
/// prepares everything for jumping back into kernel code.
///
/// # Returns
///
/// The offset of the kernel base after relocation to the base befor relocation.
///
/// # Safety
///
/// The arguments taken by this function are passed by the kernel when invoking the
/// loader. Therefore, the kernel is expected to provide valid information to us.
pub unsafe extern "C" fn load_kernel(
    kernel_base: usize,
    kernel_map: *const KernelMap,
    ini1_base: usize,
) -> usize {
    // Relocate the kernel physically in DRAM, if required.
    let (_, kernel_map) = unsafe { relocate_kernel_physically(kernel_base, kernel_map) };
    let kernel_map = unsafe { &*kernel_map };

    // check alignment of kernel map offsets
    assert_eq!(kernel_base & 0xFFF, 0, "kernel_base is not aligned");
    assert_eq!(
        kernel_map.text_start & 0xFFF,
        0,
        "text_start is not aligned"
    );
    assert_eq!(kernel_map.text_end & 0xFFF, 0, "text_end is not aligned");
    assert_eq!(
        kernel_map.rodata_start & 0xFFF,
        0,
        "rodata_start not aligned"
    );
    assert_eq!(
        kernel_map.rodata_end & 0xFFF,
        0,
        "rodata_end is not aligned"
    );
    assert_eq!(
        kernel_map.data_start & 0xFFF,
        0,
        "data_start is not aligned"
    );
    assert_eq!(kernel_map.data_end & 0xFFF, 0, "data_end is not aligned");

    // reserve 0x68000 extra bytes if requested by the kernel
    let reserved_data_size = KERNEL_DATA_SIZE
        + bsp::reserve_additional_kernel_data() as usize * ADDITIONAL_KERNEL_DATA_SIZE;

    // calculate addresses where to place INI1
    let ini1_end = kernel_base as usize + kernel_map.ini1 as usize + reserved_data_size;
    let ini1_start = ini1_end - MAX_INI1_SIZE;

    // relocate INI1 if it isn't in the right spot
    if ini1_start != ini1_base {
        // validate the INI1 binary by checking magic number and size
        let header = unsafe { &*(ini1_base as *const InitialProcessBinaryHeader) };

        if header.magic == INI1_MAGIC && header.size as usize <= MAX_INI1_SIZE {
            // valid INI1 binary, relocate!
            unsafe {
                ptr::copy(
                    ini1_base as *const u8,
                    ini1_start as *mut u8,
                    header.size as usize,
                );
            }
        } else {
            // invalid binary, so we place an invalid header at the target address,
            // which will cause a kernel panic later
            unsafe {
                ptr::write(
                    ini1_start as *mut InitialProcessBinaryHeader,
                    InitialProcessBinaryHeader::default(),
                );
            }
        }
    }

    // initialize the global page allocator
    let page_region = ini1_end;
    let page_region_size = 2 << 20;
    unsafe {
        INITAL_PAGE_ALLOCATOR.initialize(page_region);
    }

    // setup MMU with initial identity mapping
    let mut ttbr1_table = PageTableMapper::new(&INITAL_PAGE_ALLOCATOR);
    setup_initial_identity_mapping(
        &mut ttbr1_table,
        kernel_base,
        kernel_map,
        page_region,
        page_region_size,
    );

    todo!()
}

/// Identity maps the Kernel, Kernel Loader, and page region and then enables the MMU and switches
/// into virtual memory.
fn setup_initial_identity_mapping(
    ttbr1_table: &mut PageTableMapper,
    kbase: usize,
    kmap: &KernelMap,
    page_region: usize,
    page_region_size: usize,
) {
    // create ttbr0 table
    let mut ttbr0_table = PageTableMapper::new(&INITAL_PAGE_ALLOCATOR);

    // identity map kernel, loader and page region
    let rwx_attrs = || {
        use PAGE_DESCRIPTOR::*;

        let reg = InMemoryRegister::new(0);
        reg.write(UXN::True + AttrIndx.val(2) + SH::OuterShareable + AF::True);
        reg
    };

    // identity map the kernel
    ttbr0_table
        .map_many(
            PhysAddr::new(kbase),
            VirtAddr::new(kbase),
            kmap.data_end as usize,
            rwx_attrs(),
            &INITAL_PAGE_ALLOCATOR,
        )
        .unwrap();

    // identity map the loader
    let (start, size) = unsafe {
        let (start, end) = linker_symbol!(__start__, __end__);
        (start as usize, end as usize - start as usize)
    };

    ttbr0_table
        .map_many(
            PhysAddr::new(start),
            VirtAddr::new(start),
            size,
            rwx_attrs(),
            &INITAL_PAGE_ALLOCATOR,
        )
        .unwrap();

    // identity map the page region
    ttbr0_table
        .map_many(
            PhysAddr::new(page_region),
            VirtAddr::new(page_region),
            page_region_size,
            rwx_attrs(),
            &INITAL_PAGE_ALLOCATOR,
        )
        .unwrap();

    // set TTBRx registers to point to the root page tables
    TTBR0_EL1.set(ttbr0_table.root_ptr() as u64);
    TTBR1_EL1.set(ttbr1_table.root_ptr() as u64);

    // configure memory attributes (MAIR) and translation control (TCR)
    MAIR_EL1.write(
        MAIR_EL1::Attr1_Device::nonGathering_nonReordering_EarlyWriteAck
            + MAIR_EL1::Attr2_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr2_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr3_Normal_Inner::NonCacheable
            + MAIR_EL1::Attr3_Normal_Outer::NonCacheable,
    );

    TCR_EL1.write(
        TCR_EL1::T0SZ.val(25)
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::SH0::Inner
            + TCR_EL1::TG0::KiB_4
            + TCR_EL1::T1SZ.val(25)
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::SH1::Inner
            + TCR_EL1::TG1::KiB_4
            + TCR_EL1::IPS::Bits_36
            + TCR_EL1::AS::ASID16Bits,
    );

    // perform board / architecture specific setup
    unsafe {
        rt::arch_specific_setup();
    }

    // flush caches so page tables will be read once MMU is enabled
    todo!("flush caches");

    // enable the MMU!
    // FIXME: Replace with proper tock-register abstractions
    SCTLR_EL1.set(0x34D5D925);
    unsafe {
        barrier::dsb(barrier::SY);
        barrier::isb(barrier::SY);
    }
}

/// Retrieves memory layout information from the secure monitor, and adjusts the
/// kernel's physical location if necessary.
///
/// # Returns
///
/// The adjusted kernel base and kernel map pointer.
unsafe fn relocate_kernel_physically(
    kernel_base: usize,
    kernel_map: *const KernelMap,
) -> (usize, *const KernelMap) {
    match bsp::adjust_kernel_base(kernel_base) {
        Some(new_base) => unsafe {
            // The base was changed, relocate the kernel physically.
            ptr::copy(
                kernel_base as *const u8,
                new_base as *mut u8,
                (*kernel_map).data_end as usize,
            );

            // Adjust the kernel_map pointer correspondingly to the changed base.
            let diff = new_base - kernel_base;
            (
                kernel_base + diff,
                (kernel_map as *const u8).add(diff).cast::<KernelMap>(),
            )
        },
        None => (kernel_base, kernel_map),
    }
}
