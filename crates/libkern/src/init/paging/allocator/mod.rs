use core::{
    alloc::Layout,
    mem::size_of,
    ptr::{self, NonNull},
};

use crate::{addr::PhysAddr, BUILD_CONFIG};

mod cursor;

mod hole;
use self::hole::{Hole, HoleList};

/// The state managed by [`InitialPageAllocator`].
#[repr(C)]
pub struct AllocatorState {
    start_address: PhysAddr,
    end_address: PhysAddr,
    list: HoleList,
}

/// A page allocator to be used for setup during initial Kernel
/// bootstrap.
///
/// It features securely randomized page allocations by feeding
/// in a desired allocation size and obtaining the start address
/// of the allocated unit.
///
/// The safety of using this data structure intrinsically depends
/// on the absence of running MMU, so all the memory addressing
/// happens in terms of physical memory.
pub struct InitialPageAllocator {
    state: AllocatorState,
}

impl InitialPageAllocator {
    /// The minimum supported size for allocations.
    pub const MIN_SIZE: usize = size_of::<usize>() * 2;

    /// Creates a new page allocator with empty state.
    ///
    /// All attempts to allocate memory will fail until
    /// [`InitialPageAllocator::init`] was called.
    #[inline(always)]
    pub const fn new() -> Self {
        let null = unsafe { PhysAddr::new_unchecked(ptr::null_mut::<u8>()) };

        Self {
            state: AllocatorState {
                start_address: null,
                end_address: null,
                list: HoleList::empty(),
            },
        }
    }

    /// Creates a new page allocator from the given state.
    ///
    /// Depending on whether the state was already initialized,
    /// [`InitialPageAllocator::init`] might have to be called
    /// before the resulting object can be used.
    #[inline(always)]
    pub const fn with_state(state: AllocatorState) -> Self {
        Self { state }
    }

    /// Gets an immutable reference to the allocator's state.
    #[inline(always)]
    pub fn state(&self) -> &AllocatorState {
        &self.state
    }

    /// Consumes the allocator and returns its state.
    #[inline(always)]
    pub fn into_state(self) -> AllocatorState {
        self.state
    }

    /// Initializes an empty allocator to the given physical
    /// start address.
    ///
    /// # Panics
    ///
    /// Panics when the allocator is already initialized
    /// or when `start` is not aligned to [`Self::MIN_SIZE`].
    ///
    /// # Safety
    ///
    /// `start` must be a non-null memory address with the
    /// MMU disabled.
    ///
    /// Further, it must be aligned to [`Self::MIN_SIZE`]
    /// and point to an unaliased, possibly uninitialized
    /// memory region with sufficient provenance for writing
    /// to said region.
    pub unsafe fn init(&mut self, start: PhysAddr) {
        assert_eq!(
            self.state.start_address.addr(),
            0,
            "Allocator already initialized"
        );
        assert!(
            self.state.start_address.is_aligned(Self::MIN_SIZE),
            "Start address is unaligned"
        );

        self.state.start_address = start;
        self.state.start_address = start;
    }

    fn align_layout(layout: Layout) -> Layout {
        Layout::from_size_align(
            layout.size().max(Self::MIN_SIZE),
            layout.align().max(Self::MIN_SIZE),
        )
        .unwrap()
    }

    /// Allocates the given [`Layout`].
    ///
    /// The returned pointer points to the start of the
    /// allocation region and its memory contents may
    /// or may not be initialized.
    ///
    /// Returns a null pointer if allocating fails.
    ///
    /// # Panics
    ///
    /// Panics when the allocator is not already initialized.
    #[allow(clippy::result_unit_err)]
    pub fn allocate(&mut self, mut layout: Layout) -> *mut u8 {
        assert_ne!(
            self.state.start_address.addr(),
            0,
            "Allocator is uninitialized"
        );

        layout = Self::align_layout(layout);

        while !self.state.list.is_allocatable(layout) {
            unsafe {
                self.state.list.free(
                    NonNull::new_unchecked(self.state.end_address.as_mut_ptr()),
                    u64::BITS as usize * BUILD_CONFIG.page_size,
                );

                self.state.end_address = self
                    .state
                    .end_address
                    .map_addr(|addr| addr + layout.size())
                    .unwrap();
            }
        }

        let aligned_start = self.state.start_address.align_up(layout.align()).unwrap();
        //let aligned_end = self.state.end_address.align_down(layout.align()).unwrap();
        // TODO: Compute random offset for the allocation address.
        unsafe {
            self.state
                .list
                .try_allocate(aligned_start.addr(), layout.size())
                .unwrap_or(ptr::null_mut())
        }
    }

    /// Frees allocated memory of `size` bytes at `ptr`.
    ///
    /// The memory region must not be accessed past the call to
    /// this method.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid allocation returned from
    ///   [`InitialPageAllocator::allocate`].
    ///
    /// - `size` must be the correct size associated with the
    ///   allocation.
    pub unsafe fn free(&mut self, ptr: *mut u8, size: usize) {
        let ptr = NonNull::new_unchecked(ptr.cast::<Hole>());
        self.state.list.free(ptr, size)
    }
}
