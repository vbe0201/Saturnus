use cortex_a::paging::{page, PageAllocator, PhysAddr};
use libutils::{
    assert::{Assert, True},
    mem,
};

// This is a historic relict from earlier days of the kernel when randomization
// was initially implemented. The concept was to hold a 64-bit bit map where
// every bit n would correspond to `next_free_address + 0x1000 * n` being free
// or not. And when all the pages were allocated, they would add `UNIT_SIZE`
// to the next free address. With undergoing kernel changes and the introduction
// of `FreePageList`, this unit size was kept as what they previously had.
// Although not strictly required. Thanks, SciresM.
const UNIT_SIZE: usize = mem::bit_size_of::<u64>() * page::_4K;

type FreePageFramePtr = Option<&'static mut FreePageFrame>;

struct FreePageList {
    head: FreePageFramePtr,
}

struct FreePageFrame {
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
            let alloc_last_addr = mem::align_up(frame.address(), ALIGN) + SIZE - 1;
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

///
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
        Assert<{ SIZE % page::_4K == 0 }>: True,
    {
        todo!()
    }

    ///
    pub fn allocate_aligned<const SIZE: usize, const ALIGN: usize>(&mut self) -> PhysAddr
    where
        Assert<{ SIZE % page::_4K == 0 }>: True,
    {
        // Ensure that there are list nodes left for us.
        while !self.page_list.is_allocatable::<ALIGN, SIZE>() {
            unsafe {
                self.free::<{ UNIT_SIZE }>(self.next_free_address);
                self.next_free_address += UNIT_SIZE;
            }
        }

        // Find a random address and allocate memory there.
        let aligned_start = mem::align_up(self.start_address.as_usize(), ALIGN);
        let aligned_end = mem::align_down(self.next_free_address.as_usize(), ALIGN);
        loop {
            // TODO: Generate real random address.
            let random_address = aligned_start;
            if self.try_allocate::<SIZE>(random_address).is_ok() {
                return PhysAddr::new(random_address);
            }
        }
    }
}

unsafe impl PageAllocator<{ page::_4K }> for InitialPageAllocator {
    #[inline]
    fn allocate<const SIZE: usize>(&mut self) -> Option<PhysAddr>
    where
        Assert<{ SIZE % page::_4K == 0 }>: True,
    {
        Some(self.allocate_aligned::<SIZE, SIZE>())
    }

    unsafe fn free<const SIZE: usize>(&mut self, addr: PhysAddr)
    where
        Assert<{ SIZE % page::_4K == 0 }>: True,
    {
        todo!()
    }
}

///
pub struct InitialPageTable;
