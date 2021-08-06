use crate::{
    paging::{PageTableMapper, PhysAddr, VirtAddr, PAGE_DESCRIPTOR},
    INITAL_PAGE_ALLOCATOR,
};
use core::ptr;
use cortex_a::{
    asm::barrier,
    registers::{self, MAIR_EL1, SCTLR_EL1, TCR_EL1, TTBR0_EL1, TTBR1_EL1},
};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

/// The maximum size of the INI1 section (12 MiB).
pub const MAX_INI1_SIZE: usize = 12 << 20;

/// The amount of bytes that will be reserved for the kernel to use and
/// thus won't be overwritten by the loader.
pub const KERNEL_DATA_SIZE: usize = 0x1728000;

/// Amount of memory that will be reserved *additionally* to the [`KERNEL_DATA_SIZE`] if
/// the kernel requests a larger amount reserved data.
pub const ADDITIONAL_KERNEL_DATA_SIZE: usize = 0x68000;

/// Implementer ID of an ARM limited processor.
pub const ARM_LIMITED_ID: u8 = 0x41;

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

/// Relocates the kernel to a random base address, identity maps the kernel and prepares everything
/// for jumping back into the kernel.
///
/// # Returns
///
/// The relocated base address of the kernel.
pub unsafe extern "C" fn load_kernel(kbase: usize, kmap: &KernelMap, ini1_base: usize) -> usize {
    // relocate the kernel physically, if required
    let (kbase, kmap) = unsafe { relocate_kernel_physically(kbase, kmap) };

    // check alignment of kernel map offsets
    assert_eq!(kbase & 0xFFF, 0, "kernel_base is not aligned");
    assert_eq!(kmap.text_start & 0xFFF, 0, "text_start is not aligned");
    assert_eq!(kmap.text_end & 0xFFF, 0, "text_end is not aligned");
    assert_eq!(kmap.rodata_start & 0xFFF, 0, "rodata_start not aligned");
    assert_eq!(kmap.rodata_end & 0xFFF, 0, "rodata_end is not aligned");
    assert_eq!(kmap.data_start & 0xFFF, 0, "data_start is not aligned");
    assert_eq!(kmap.data_end & 0xFFF, 0, "data_end is not aligned");

    // reserve 0x68000 extra bytes if requested by the kernel
    let reserved_data_size =
        KERNEL_DATA_SIZE + should_reserve_additional_data() as usize * ADDITIONAL_KERNEL_DATA_SIZE;

    // calculate addresses where to place INI1
    let ini1_end = kbase as usize + kmap.ini1 as usize + reserved_data_size;
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
    setup_initial_identity_mapping(&mut ttbr1_table, kbase, kmap, page_region, page_region_size);

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
        extern "C" {
            static __start__: u8;
            static __end__: u8;
        }

        let start = &__start__ as *const _ as usize;
        let end = &__end__ as *const _ as usize;
        (start, end - start)
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

    // check which CPU we are running, and configure CPUECTLR, CPUACTLR appropriately
    let manufacture_id = registers::MIDR_EL1.get();
    let implementer = (manufacture_id >> 24) as u8;

    todo!("Save X19-X30 + SP, save context struct in TPIDR_EL1.");

    if implementer == ARM_LIMITED_ID {
        // FIXME: Implement this
    }

    todo!("Verify that TPIDR_EL1 is still set.");

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

/// This retrieves memory layout information from the secure monitor,
/// and adjusts the kernel's physical location if necessary.
///
/// # Returns
///
/// The adjusted kernel base and kernel map pointer.
unsafe fn relocate_kernel_physically(
    kernel_base: usize,
    kernel_map: &KernelMap,
) -> (usize, &KernelMap) {
    // if the base address was adjusted, move the kernel to the new base
    // and return the new pointers
    if let Some(adjusted_base) = get_adjusted_kernel_base(kernel_base) {
        // copy kernel data to new location
        // FIXME: This can probably be adjusted to copy words instead of bytes
        unsafe {
            ptr::copy(
                kernel_base as *const u8,
                adjusted_base as *mut u8,
                kernel_map.data_end as usize,
            )
        };

        // calculate the new kernel base and kernel map pointer
        let diff = adjusted_base - kernel_base;
        let kernel_map = unsafe {
            let ptr = kernel_map as *const KernelMap as *const u8;
            let ptr = ptr.add(diff);
            &*ptr.cast::<KernelMap>()
        };

        (kernel_base + diff, kernel_map)
    } else {
        (kernel_base, kernel_map)
    }
}

/// This sees how much more memory is available than expected, and relocates the kernel accordingly.
///
/// # Returns
///
/// `None` if the kernel does not require any relocation, otherwise `Some` with the adjusted base
/// address.
#[allow(clippy::diverging_sub_expression, unused_variables, unreachable_code)]
fn get_adjusted_kernel_base(base: usize) -> Option<usize> {
    // temporary to make the panics go away
    return None;

    // read DRAM size information from memory controller
    let dram_size_from_mc: usize = todo!("tegra210 implementation missing");

    // read DRAM size information from Secure Monitor KernelConfiguration
    let memory_type: usize = todo!("tegra210 implementation missing");

    // convert memory type to size of memory
    let dram_size_from_kernel_cfg = match memory_type {
        // MemoryType_6GB = 1
        1 => 6 << 30,
        // MemoryType_8GB = 2
        2 => 8 << 30,
        // MemoryType_4G  = 0 (default case)
        _ => 4 << 30,
    };

    // on normal systems, these should be equal and kernel will not be relocated
    if dram_size_from_mc < 2 * dram_size_from_kernel_cfg {
        Some(base + (dram_size_from_mc - dram_size_from_kernel_cfg) / 2)
    } else {
        None
    }
}

/// This functions checks a flag from the KernelConfiguration.
fn should_reserve_additional_data() -> bool {
    // FIXME: This needs to be implemented correctly, requires tegra210 crate
    false
}
