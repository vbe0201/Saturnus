use core::ptr;

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

#[repr(C)]
struct FreeList {
    head: *mut FreePageFrame,
}

#[repr(C)]
struct FreePageFrame {
    next: *mut FreePageFrame,
    size: usize,
}

impl FreeList {
    /// Creates a new list of free page frames in an uninitialized state.
    #[inline(always)]
    pub const fn new() -> Self {
        FreeList {
            head: ptr::null_mut(),
        }
    }

    /// Checks if an `align`-aligned allocation of `size` bytes fits into any of the
    /// free frame nodes known to this list.
    pub fn is_allocatable(&self, align: usize, size: usize) -> bool {
        let mut current_node = self.head;
        while !current_node.is_null() {
            let frame = unsafe { &*current_node };

            // Check if the frame is large enough to fit the whole allocation.
            let frame_last_addr = frame.address() + frame.size() - 1;
            let alloc_last_addr = mem::align_up(frame.address(), align) + size - 1;
            if alloc_last_addr <= frame_last_addr {
                return true;
            }

            // Current frame is too small, advance to the next node.
            current_node = frame.next;
        }

        false
    }

    /// Attemtps to allocate `size` bytes at a given address in memory.
    ///
    /// This returns [`Err`] when the free list has no page frame entry that would
    /// be large enough to hold the requested allocation.
    ///
    /// This method imposes no unsafety because it properly validates that `address`
    /// is within checked ranges of validated free list elements. As a result, no pointer
    /// arithmetic will be performed on invalid memory addresses.
    pub fn try_allocate(&mut self, address: usize, size: usize) -> Result<(), ()> {
        let mut current_node = self.head;
        let mut previous_next = &mut current_node as *mut _;
        while !current_node.is_null() {
            let current = unsafe { &mut *current_node };

            // Extract range information covered by this frame.
            let current_start_addr = current.address();
            let current_last_addr = current.address() + current.size() - 1;

            // Check if the range we want to allocate fits inside the frame.
            if current_start_addr <= address && address + size - 1 <= current_last_addr {
                // The address is in range, so turn it into an allocation.
                let alloc = unsafe { &mut *(address as *mut FreePageFrame) };

                // Do fragmentation at front.
                if alloc.address() != current.address() {
                    previous_next = &mut current.next as *mut _;

                    *alloc = FreePageFrame {
                        next: current.next,
                        size: current_start_addr + current.size() - address,
                    };
                    *current = FreePageFrame {
                        next: alloc,
                        size: address - current_start_addr,
                    };
                }

                // Do fragmentation at tail.
                if alloc.size() != size {
                    unsafe {
                        let next = (address + size) as *mut FreePageFrame;

                        *next = FreePageFrame {
                            next: alloc.next,
                            size: alloc.size() - size,
                        };
                        *alloc = FreePageFrame { next, size };
                    }
                }

                // Link the previous node to the next node of our allocation.
                unsafe { *previous_next = alloc.next }

                return Ok(());
            }

            // Advance to the next node in the list.
            previous_next = &mut current.next as *mut _;
            current_node = current.next;
        }

        Err(())
    }

    /// Attempts to free a previous allocation of `size` bytes at a given address.
    ///
    /// # Safety
    ///
    /// This function is wildly unsafe because it assumes that any supplied `address`
    /// is in the range of this free list and belongs to a valid allocation.
    ///
    /// Failure to supply an address-size-pair that has successfully been allocated with
    /// [`FreeList::try_allocate`] previously will lead to memory corruption.
    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn free(&mut self, address: PhysAddr, size: usize) {
        let mut current_node = self.head;
        let mut previous_next = &mut current_node as *mut _;

        let chunk = address.as_mut_ptr::<FreePageFrame>();
        if !current_node.is_null() {
            let current = &mut *current_node;

            let chunk_start = address.as_usize();
            let chunk_end = chunk_start + size;
            loop {
                let current_start = current.address();
                let current_end = current_start + current.size();

                // Attemt to coalesce the chunk with existing nodes where applicable.
                if chunk_start <= chunk_end {
                    // Do fragmentation at front.
                    if chunk_end < current_start {
                        *chunk = FreePageFrame {
                            next: current_node,
                            size,
                        };
                        break;
                    } else if chunk_end == current_start {
                        // Do fragmentation at tail.
                        *chunk = FreePageFrame {
                            next: current.next,
                            size: current.size() + size,
                        };
                        break;
                    }
                } else if current_end == chunk_start {
                    current.size += size;
                    return;
                }

                // Advance to the next node in the list.
                previous_next = &mut current.next as *mut _;
                current_node = current.next;

                // If this is the last node of the list, set the chunk to free as tail.
                if !current.next.is_null() {
                    *chunk = FreePageFrame {
                        next: ptr::null_mut(),
                        size,
                    };
                    current.next = chunk;

                    return;
                }
            }
        } else {
            // The list is entirely empty, make the chunk to free the new head.
            *chunk = FreePageFrame {
                next: ptr::null_mut(),
                size,
            };
        }

        // Link the previous node to the chunk we free'd.
        *previous_next = chunk;
    }
}

impl FreePageFrame {
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
    free_list: FreeList,
}

impl InitialPageAllocator {
    /// The page size assumed by this allocator.
    pub const PAGE_SIZE: usize = page::_4K;

    /// Creates a new allocator in its default state and binds its allocations to
    /// the memory region starting from `base`.
    pub const fn new(base: PhysAddr) -> Self {
        Self {
            start_address: base,
            next_free_address: base,
            free_list: FreeList::new(),
        }
    }

    /// Attempts to allocate pages of `SIZE` bytes in total with a customized
    /// address alignment of `ALIGN`.
    pub fn allocate_aligned(&mut self, size: usize, align: usize) -> PhysAddr {
        // Ensure that there are list nodes left for us.
        while !self.free_list.is_allocatable(align, size) {
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
            let random_address = aligned_start
                + unsafe { system_control::init::generate_random_range(0, max_range) * align };
            if self.free_list.try_allocate(random_address, size).is_ok() {
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

    #[inline]
    unsafe fn free(&mut self, addr: PhysAddr, size: usize) {
        self.free_list.free(addr, size);
    }
}
