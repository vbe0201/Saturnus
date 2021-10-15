use cortex_a::{
    paging::{granule, page, PageAllocator, PhysAddr},
    utils,
};

type FreePageFramePtr = Option<&'static mut FreePageFrame>;

pub struct FreePageList {
    head: FreePageFramePtr,
}

pub struct FreePageFrame {
    next: FreePageFramePtr,
    size: usize,
}

impl FreePageList {
    /// Constructs a new page frame list without any nodes.
    pub const fn new() -> Self {
        FreePageList { head: None }
    }

    /// Checks if the list contains a free node that is large enough to hold an
    /// allocation of `size` bytes, aligned to `align` bytes.
    ///
    /// This must return [`true`] as a precondition prior to an allocation.
    pub fn is_allocatable<const ALIGN: usize, const SIZE: usize>(&self) -> bool {
        let mut current_frame = &self.head;
        while let Some(frame) = current_frame {
            // Check if the frame is large enough to fit the whole allocation.
            let frame_last_addr = frame.address() + frame.size() - 1;
            let alloc_last_addr = utils::align_up(frame.address(), ALIGN) + SIZE - 1;
            if alloc_last_addr <= frame_last_addr {
                return true;
            }

            // Advance to the next frame node in the list.
            current_frame = frame.next();
        }

        false
    }
}

impl FreePageFrame {
    /// Gets a reference to the next element pointer linked to this node.
    #[inline]
    pub fn next(&self) -> &FreePageFramePtr {
        &self.next
    }

    /// Gets the size of this page frame.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets the address of this page frame in memory.
    #[inline]
    pub fn address(&self) -> usize {
        self as *const _ as usize
    }
}

pub struct InitialPageAllocator {
    start_address: PhysAddr,
    next_free_address: PhysAddr,
    page_list: FreePageList,
}

impl InitialPageAllocator {
    ///
    pub const fn new() -> Self {
        Self {
            start_address: PhysAddr::zero(),
            next_free_address: PhysAddr::zero(),
            page_list: FreePageList::new(),
        }
    }

    ///
    #[inline(always)]
    pub const fn initialize(&mut self, address: usize) {
        self.start_address = PhysAddr::new(address);
        self.next_free_address = PhysAddr::new(address);
    }

    fn try_allocate<const SIZE: usize>(&mut self, address: usize) -> Result<(), ()>
    where
        page::PageSize<SIZE>: page::SupportedPageSize,
    {
        todo!()
    }

    ///
    pub fn allocate_aligned<const SIZE: usize, const ALIGN: usize>(&mut self) -> PhysAddr
    where
        page::PageSize<SIZE>: page::SupportedPageSize,
    {
        // Ensure that there are list nodes left for us.
        while !self.page_list.is_allocatable::<ALIGN, SIZE>() {
            unsafe {
                self.free::<{ granule::_4K }>(self.next_free_address);
                self.next_free_address += granule::_4K;
            }
        }

        // Find a random address and allocate memory there.
        let aligned_start = utils::align_up(self.start_address.as_usize(), ALIGN);
        let aligned_end = utils::align_down(self.next_free_address.as_usize(), ALIGN);
        loop {
            // TODO: Generate real random address.
            let random_address = aligned_start;
            if self.try_allocate::<SIZE>(random_address).is_ok() {
                return PhysAddr::new(random_address);
            }
        }
    }
}

unsafe impl PageAllocator for InitialPageAllocator {
    #[inline]
    fn allocate<const SIZE: usize>(&mut self) -> Option<PhysAddr>
    where
        page::PageSize<SIZE>: page::SupportedPageSize,
    {
        Some(self.allocate_aligned::<SIZE, SIZE>())
    }

    unsafe fn free<const SIZE: usize>(&mut self, addr: PhysAddr)
    where
        page::PageSize<SIZE>: page::SupportedPageSize,
    {
        todo!()
    }
}

pub struct InitialPageTable;
