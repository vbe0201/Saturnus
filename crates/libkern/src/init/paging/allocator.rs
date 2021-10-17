use cortex_a::paging::{page, PageAllocator, PhysAddr};
use libutils::mem;

use crate::system_control;

// This is a historic relict from earlier days of the kernel when randomization
// were initially implemented. The concept was to hold a 64-bit bitmap where
// every bit n would correspond to `next_free_address + 0x100 * n` being free
// or not. And when all the pages were allocated, they would add `UNIT_SIZE`
// to the next free address. With further improvements and the introduction of
// `FreePageList`, this unit size was kept as what they previously used already.
// Despite not being strictly required. Thanks, SciresM.
const UNIT_SIZE: usize = mem::bit_size_of::<u64>() * InitialPageAllocator::PAGE_SIZE;

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

    /// Checks if the list contains a free node that is large enough to fit an
    /// allocation of `size` bytes, aligned to `alignn` bytes.
    ///
    /// This must return [`true`] as a precondition for an allocation to succeed.
    pub fn is_allocatable(&self, align: usize, size: usize) -> bool {
        let mut current_frame = &self.head;
        while let Some(frame) = current_frame {
            // Check if the frame is large enough to fit the whole allocation.
            let frame_last_addr = frame.address() + frame.size() - 1;
            let alloc_last_addr = mem::align_up(frame.address(), align) + size - 1;
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
    /// Gets a reference to the next element pointer linked to this frame node.
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

/// An allocator to be used for page setup during initial kernel bootstrap.
///
/// It features securely randomized page allocations by feeding in a desired
/// allocation size and obtaining the start address of the allocated unit.
///
/// Note that due to the absence of virtual memory, all addresses here are
/// of physical nature. This allocator thus is unfit when virtual addresses
/// are expected.
///
/// This uses the default page size of 4KiB.
pub struct InitialPageAllocator {
    start_address: PhysAddr,
    next_free_address: PhysAddr,
    page_list: FreePageList,
}

impl InitialPageAllocator {
    /// The page size assumed by this allocator.
    pub const PAGE_SIZE: usize = page::_4K;

    /// Creates a new allocator in its default state.
    ///
    /// The resulting object needs to be initialized through a corresponding
    /// method before it can be used.
    pub const fn new() -> Self {
        Self {
            start_address: PhysAddr::zero(),
            next_free_address: PhysAddr::zero(),
            page_list: FreePageList::new(),
        }
    }

    /// Initializes the page allocator to a given physical base address in
    /// memory where allocations can be placed.
    ///
    /// # Safety
    ///
    /// This function may make the allocator cause memory corruption when `base`
    /// points to a place that is simultaneously used for something else.
    ///
    /// Further, `base` must be page-aligned.
    #[inline(always)]
    pub const unsafe fn initialize(&mut self, base: usize) {
        self.start_address = PhysAddr::new(base);
        self.next_free_address = PhysAddr::new(base);
    }

    fn try_allocate(&mut self, address: usize, size: usize) -> Result<(), ()> {
        todo!()
    }

    /// Attempts to allocate pages of `SIZE` bytes in total with a customized
    /// address alignment of `ALIGN`.
    pub fn allocate_aligned(&mut self, size: usize, align: usize) -> PhysAddr {
        // Ensure that there are list nodes left for us.
        while !self.page_list.is_allocatable(align, size) {
            unsafe {
                self.free(self.next_free_address, UNIT_SIZE);
                self.next_free_address += UNIT_SIZE;
            }
        }

        // Find a random address and allocate memory there.
        let aligned_start = mem::align_up(self.start_address.as_usize(), align);
        let aligned_end = mem::align_down(self.next_free_address.as_usize(), align);
        let max_range = ((aligned_end - aligned_start) / align) - 1;
        loop {
            let random_address =
                aligned_start + system_control::init::generate_random_range(0, max_range) * align;
            if self.try_allocate(random_address, size).is_ok() {
                return unsafe { PhysAddr::new_unchecked(random_address) };
            }
        }
    }
}

unsafe impl PageAllocator for InitialPageAllocator {
    #[inline]
    fn allocate(&mut self, size: usize) -> Option<PhysAddr> {
        Some(self.allocate_aligned(size, size))
    }

    unsafe fn free(&mut self, addr: PhysAddr, size: usize) {
        todo!()
    }
}
