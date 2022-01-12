//!

use core::{arch::asm, marker::PhantomData, mem};

use cortex_a::{
    paging::{
        page::{PageSize, SupportedPageSize},
        table_entry::{l1_block_size, max_table_descriptors},
        PageAllocator, PhysAddr, VirtAddr,
    },
    registers::{TCR_EL1, TTBR0_EL1, TTBR1_EL1},
};
use libutils::mem::align_down;
use tock_registers::interfaces::Readable;

/// The page table to be used during initial kernel bootstrap.
///
/// It internally manages two L1 tables and operates on them.
/// The page size is inferred from the allocator it is used with.
pub struct InitialPageTable<PA: PageAllocator> {
    l1_tables: [PhysAddr; 2],
    num_blocks: [u32; 2],

    _pa: PhantomData<fn() -> PA>,
}

#[inline]
unsafe fn clear_page_region_volatile(address: PhysAddr, size: usize) {
    debug_assert!(address.is_aligned(size) && size >= mem::size_of::<u64>() * 2);

    // SAFETY: The MMU is not configured at this point. This code is
    // used in conjunction with allocation of the initial page table.
    asm!(
        r#"
        1:
            stp xzr, xzr, [{page_ptr}], #16
            cmp {page_ptr}, {page_end}
            b.eq 2f
            b 1b

        2:
    "#,
        page_ptr = in(reg) address.as_mut_ptr::<u64>(),
        page_end = in(reg) address.as_usize() + size,
    );
}

impl<PA: PageAllocator> InitialPageTable<PA>
where
    PageSize<{ PA::PAGE_SIZE }>: SupportedPageSize,
{
    /// Tries to allocate new page tables using the given allocator.
    ///
    /// The number of L1 blocks will be set to what fits between the region
    /// spanned by `start` and `end`.
    ///
    /// This may return [`None`] when the required memory allocation fails.
    pub fn new(start: VirtAddr, end: VirtAddr, allocator: &mut PA) -> Option<Self> {
        // Allocate the L1 page tables.
        let l1_tables = [
            Self::allocate_table(allocator)?,
            Self::allocate_table(allocator)?,
        ];

        // Set the page table blocks.
        let max_table_entries = max_table_descriptors::<{ PA::PAGE_SIZE }>() as usize;
        let l1_size = l1_block_size::<{ PA::PAGE_SIZE }>() as usize;
        let num_blocks = [
            max_table_entries as u32,
            (((end.as_usize() / l1_size) & (max_table_entries - 1))
                - ((start.as_usize() / l1_size) & (max_table_entries - 1))
                + 1) as u32,
        ];

        debug_assert!(0 < num_blocks[1] && num_blocks[1] <= max_table_entries as u32);

        Some(Self {
            l1_tables,
            num_blocks,

            _pa: PhantomData,
        })
    }

    #[inline(always)]
    fn allocate_table(allocator: &mut PA) -> Option<PhysAddr> {
        let address = allocator.allocate(PA::PAGE_SIZE)?;
        unsafe { clear_page_region_volatile(address, PA::PAGE_SIZE) }

        Some(address)
    }
}

impl<PA: PageAllocator> Default for InitialPageTable<PA>
where
    PageSize<{ PA::PAGE_SIZE }>: SupportedPageSize,
{
    fn default() -> Self {
        // Read the L1 tables from the Translation Table Base registers.
        let l1_tables = [
            PhysAddr::new(align_down(unsafe { TTBR0_EL1.get() as _ }, PA::PAGE_SIZE)),
            PhysAddr::new(align_down(unsafe { TTBR1_EL1.get() as _ }, PA::PAGE_SIZE)),
        ];

        // Read the sizes of the table memory regiosn and subdivide them into blocks.
        let num_blocks = [
            unsafe { TCR_EL1.read(TCR_EL1::T0SZ) / l1_block_size::<{ PA::PAGE_SIZE }>() } as u32,
            unsafe { TCR_EL1.read(TCR_EL1::T1SZ) / l1_block_size::<{ PA::PAGE_SIZE }>() } as u32,
        ];

        assert!(0 < num_blocks[0] && num_blocks[0] <= max_table_descriptors::<{ PA::PAGE_SIZE }>());
        assert!(0 < num_blocks[1] && num_blocks[1] <= max_table_descriptors::<{ PA::PAGE_SIZE }>());

        Self {
            l1_tables,
            num_blocks,

            _pa: PhantomData,
        }
    }
}
