use super::{
    granule::{self, Granule, GranuleSupportsPage, SupportedGranule},
    page::{self, PageSize, SupportedPageSize},
    FrameAllocator, MapError, Page, PageFlags, PhysAddr, PhysFrame, VirtAddr,
};
use core::ptr::NonNull;

/// Provides a way to translate physical addresses to virtual addresses.
///
/// # Safety
///
/// The returned virtual address must point to the same location
/// as the given physical address.
pub unsafe trait PhysAddrTranslator {
    /// Translate a physical address to a virtual address.
    fn translate(paddr: PhysAddr) -> VirtAddr;
}

pub struct PageTable<P: PhysAddrTranslator, A: FrameAllocator, const GRANULE: usize>
where
    Granule<GRANULE>: SupportedGranule,
{
    root: NonNull<Entry>,
    translate: P,
    allocator: A,
}

/// Operations on page tables with 4 KiB granule
impl<P: PhysAddrTranslator, A> PageTable<P, A, { granule::_4K }>
where
    Granule<{ granule::_4K }>: SupportedGranule,
    A: FrameAllocator,
{
    /// Creates a new page table.
    ///
    /// This method will allocate the level 0 page table using the given allocator
    /// and may panic if the allocation fails.
    pub fn new(translate: P, allocator: A) -> Self {
        Self {
            root: Self::allocate_table(&allocator).expect("initial page allocation failed"),
            translate,
            allocator,
        }
    }

    /// Creates a new mapping in this page table.
    pub fn map<const S: usize>(
        &mut self,
        page: Page<S>,
        frame: PhysFrame<S>,
        flags: PageFlags,
    ) -> Result<(), MapError>
    where
        PageSize<S>: SupportedPageSize,
        Granule<{ granule::_4K }>: GranuleSupportsPage<S>,
    {
        match S {
            page::_4K => {
                let vaddr = page.start();

                let l0_index = (vaddr.as_usize() >> 39) & 0x1FF;
                let l1_table = unsafe { self.get_next_table(self.root, l0_index).unwrap() };

                let l1_index = (vaddr.as_usize() >> 30) & 0x1FF;
                let l2_table = unsafe { self.get_next_table(l1_table, l1_index)? };

                let l2_index = (vaddr.as_usize() >> 21) & 0x1FF;
                let l3_table = unsafe { self.get_next_table(l2_table, l2_index)? };

                let l3_index = (vaddr.as_usize() >> 12) & 0x1FF;

                // map the entry in the level 3 table to the given physical frame
                let entry = unsafe { &mut *l3_table.as_ptr().add(l3_index) };
                entry.set_page(frame.start(), flags);

                Ok(())
            }
            page::_2M => {
                let vaddr = page.start();

                let l0_index = (vaddr.as_usize() >> 39) & 0x1FF;
                let l1_table = unsafe { self.get_next_table(self.root, l0_index).unwrap() };

                let l1_index = (vaddr.as_usize() >> 30) & 0x1FF;
                let l2_table = unsafe { self.get_next_table(l1_table, l1_index)? };

                let l2_index = (vaddr.as_usize() >> 21) & 0x1FF;

                // map the entry in the level 3 table to the given physical frame
                let entry = unsafe { &mut *l2_table.as_ptr().add(l2_index) };
                entry.set_block(frame.start(), flags);

                Ok(())
            }
            page::_1G => {
                let vaddr = page.start();

                let l0_index = (vaddr.as_usize() >> 39) & 0x1FF;
                let l1_table = unsafe { self.get_next_table(self.root, l0_index).unwrap() };
                let l1_index = (vaddr.as_usize() >> 30) & 0x1FF;

                // map the entry in the level 3 table to the given physical frame
                let entry = unsafe { &mut *l1_table.as_ptr().add(l1_index) };
                entry.set_block(frame.start(), flags);

                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Read the entry at the given `idx`, and either walk the table if it's a table entry,
    /// or create a new table if the entry is unused.
    unsafe fn get_next_table(
        &mut self,
        table: NonNull<Entry>,
        idx: usize,
    ) -> Result<NonNull<Entry>, MapError> {
        let entry = unsafe { &mut *table.as_ptr().add(idx) };

        // if the entry is already mapped as a block, return `None`
        if entry.is_block() {
            return Err(MapError::PageAlreadyMapped);
        }

        // if the entry is unused, allocate a new table
        let new_table = if entry.is_unused() {
            // FIXME: better handling of allocation failures
            // allocate new table and make the current entry point to it
            let new_table =
                Self::allocate_table(&self.allocator).ok_or(MapError::PageAllocationFailed)?;
            entry.set_table(new_table);
            Some(new_table)
        } else {
            None
        };

        // either return newly created table, or get table from the entry
        Ok(new_table.unwrap_or_else(|| entry.table()))
    }

    fn allocate_table(allocator: &A) -> Option<NonNull<Entry>> {
        let page = allocator
            .allocate::<{ page::_4K }>()?
            .cast::<[Entry; 512]>();
        let page = unsafe { NonNull::new_unchecked(page.as_ptr() as *mut Entry) };
        unsafe { page.as_ptr().write_bytes(0, 512) };
        Some(page)
    }
}

/// Wrapper type around a page table entry.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Entry(u64);

impl Entry {
    pub const TABLE: u64 = 0b11;
    pub const PAGE: u64 = 0b11;
    pub const BLOCK: u64 = 0b01;
    pub const UNUSED: u64 = 0b00;

    pub const AF: u64 = 1 << 10;

    pub const ADDRESS_MASK: u64 = 0xFFFF_FFFF_F000;

    /// Make this entry be a page entry and point it to the given address.
    #[inline]
    fn set_page(&mut self, paddr: PhysAddr, flags: PageFlags) {
        self.0 = (paddr.as_usize() as u64 & Entry::ADDRESS_MASK)
            | flags.bits()
            | Entry::AF
            | Entry::PAGE;
    }

    /// Make this entry be a page entry and point it to the given address.
    #[inline]
    fn set_block(&mut self, paddr: PhysAddr, flags: PageFlags) {
        self.0 = (paddr.as_usize() as u64 & Entry::ADDRESS_MASK)
            | flags.bits()
            | Entry::AF
            | Entry::BLOCK;
    }

    /// Make this entry point to the `new_table`.
    #[inline]
    fn set_table(&mut self, new_table: NonNull<Entry>) {
        self.0 = (new_table.as_ptr() as u64 & Entry::ADDRESS_MASK) | Entry::TABLE;
    }

    /// Get the table this entry points to
    #[inline]
    fn table(&mut self) -> NonNull<Entry> {
        let addr = self.0 & Entry::ADDRESS_MASK;
        NonNull::new(addr as *mut _).unwrap()
    }

    /// Check if this entry is unused.
    #[inline]
    fn is_unused(self) -> bool {
        self.0 & 0b1 == Entry::UNUSED
    }

    /// Check if this entry is a block entry.
    #[inline]
    fn is_block(self) -> bool {
        self.0 & 0b11 == Entry::BLOCK
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use std::alloc::{Allocator, Global, Layout};

    unsafe impl<A: Allocator> FrameAllocator for A {
        fn allocate<const SIZE: usize>(&self) -> Option<NonNull<[u8; SIZE]>> {
            (*self)
                .allocate(Layout::from_size_align(SIZE, SIZE).unwrap())
                .ok()
                .map(NonNull::cast)
        }

        unsafe fn deallocate<const SIZE: usize>(&self, ptr: NonNull<u8>) {
            unsafe {
                (*self).deallocate(ptr, Layout::from_size_align(SIZE, SIZE).unwrap());
            }
        }
    }

    struct Foo;
    unsafe impl PhysAddrTranslator for Foo {
        fn translate(paddr: PhysAddr) -> VirtAddr {
            VirtAddr::new(paddr.as_usize())
        }
    }

    #[test]
    fn it_works() {
        let mut table = PageTable::<Foo, Global, { granule::_4K }>::new(Foo, Global);

        let page = Page::containing_address(VirtAddr::new(0xABCD));
        let frame = PhysFrame::containing_address(PhysAddr::new(0xABCD));
        table
            .map::<{ page::_4K }>(page, frame, PageFlags::empty())
            .unwrap();
    }
}
