use core::{arch::asm, marker::PhantomData, mem};

use cortex_a::{
    paging::{
        page::{PageSize, SupportedPageSize},
        table_entry::*,
        PageAllocator, PhysAddr, VirtAddr,
    },
    registers::{TCR_EL1, TTBR0_EL1, TTBR1_EL1},
};
use libutils::{
    bits,
    mem::{align_down, is_aligned},
};
use tock_registers::interfaces::Readable;

use super::InitialPageAllocator;

/// The page table to be used during initial kernel bootstrap.
///
/// It internally manages two L1 tables and operates on them.
/// The page size is inferred from the allocator it is used with.
pub struct InitialPageTable<PA: PageAllocator> {
    l1_tables: [PhysAddr; 2],
    num_blocks: [usize; 2],

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
        page_ptr = in(reg) address.as_usize(),
        page_end = in(reg) address.as_usize() + size,
    );
}

impl InitialPageTable<InitialPageAllocator> {
    /// Tries to allocate new page tables using the given allocator.
    ///
    /// The number of L1 blocks will be set to what fits between the region
    /// spanned by `start` and `end`.
    ///
    /// This may return [`None`] when the required memory allocation fails.
    pub fn new(
        start: VirtAddr,
        end: VirtAddr,
        allocator: &mut InitialPageAllocator,
    ) -> Option<Self> {
        // Allocate the L1 page tables.
        let l1_tables = [
            Self::allocate_table(allocator)?,
            Self::allocate_table(allocator)?,
        ];

        // Set the page table blocks.
        let max_table_entries = max_table_descriptors::<{ InitialPageAllocator::PAGE_SIZE }>();
        let l1_size = l1_block_size::<{ InitialPageAllocator::PAGE_SIZE }>() as usize;
        let num_blocks = [
            max_table_entries,
            ((end.as_usize() / l1_size) & (max_table_entries - 1))
                - ((start.as_usize() / l1_size) & (max_table_entries - 1))
                + 1,
        ];

        debug_assert!(0 < num_blocks[1] && num_blocks[1] <= max_table_entries);

        Some(Self {
            l1_tables,
            num_blocks,

            _pa: PhantomData,
        })
    }

    #[inline]
    fn allocate_table(allocator: &mut InitialPageAllocator) -> Option<PhysAddr> {
        let address = allocator.allocate(InitialPageAllocator::PAGE_SIZE)?;
        unsafe { clear_page_region_volatile(address, InitialPageAllocator::PAGE_SIZE) }

        Some(address)
    }

    #[inline]
    fn get_l1_descriptor(&mut self, address: VirtAddr) -> &mut L1PageTableDescriptor {
        let idx = (address.as_usize() >> (bits::bit_size_of::<VirtAddr>() - 1)) & 1;
        let l1_block_size = l1_block_size::<{ InitialPageAllocator::PAGE_SIZE }>() as usize;

        unsafe {
            &mut *self.l1_tables[idx]
                .as_mut_ptr::<L1PageTableDescriptor>()
                .add((address.as_usize() / l1_block_size) & (self.num_blocks[idx] - 1))
        }
    }

    #[inline]
    fn get_l2_descriptor(
        &mut self,
        entry: &mut L1PageTableDescriptor,
        address: VirtAddr,
    ) -> &mut L2PageTableDescriptor {
        let l2_block_size = l2_block_size::<{ InitialPageAllocator::PAGE_SIZE }>() as usize;
        let max_table_entries = max_table_descriptors::<{ InitialPageAllocator::PAGE_SIZE }>();

        unsafe {
            &mut *entry
                .next_table()
                .as_mut_ptr::<L2PageTableDescriptor>()
                .add((address.as_usize() / l2_block_size) & (max_table_entries - 1))
        }
    }

    #[inline]
    fn get_l3_descriptor(
        &mut self,
        entry: &mut L2PageTableDescriptor,
        address: VirtAddr,
    ) -> &mut L3PageTableDescriptor {
        let l3_block_size = l3_block_size::<{ InitialPageAllocator::PAGE_SIZE }>() as usize;
        let max_table_entries = max_table_descriptors::<{ InitialPageAllocator::PAGE_SIZE }>();

        unsafe {
            &mut *entry
                .next_table()
                .as_mut_ptr::<L3PageTableDescriptor>()
                .add((address.as_usize() / l3_block_size) & (max_table_entries - 1))
        }
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
            unsafe { TCR_EL1.read(TCR_EL1::T0SZ) / l1_block_size::<{ PA::PAGE_SIZE }>() } as _,
            unsafe { TCR_EL1.read(TCR_EL1::T1SZ) / l1_block_size::<{ PA::PAGE_SIZE }>() } as _,
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
