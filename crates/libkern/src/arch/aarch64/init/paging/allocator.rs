use core::{
    mem::{align_of, MaybeUninit},
    num::NonZeroUsize,
    ptr::{self, NonNull},
    slice,
};

use utils::align::{align_up, is_aligned};

use crate::{addr::PhysAddr, BUILD_CONFIG};

#[derive(Clone, Copy)]
#[repr(C)]
struct FreePageNode {
    next: Option<NonNull<Self>>,
    size: usize,
}

const _: () = assert!(align_of::<FreePageNode>() == align_of::<*const ()>());

/// Representation of the state tracked by [`InitialPageAllocator`].
#[repr(C)]
pub struct AllocatorState {
    start_address: PhysAddr,
    end_address: PhysAddr,
    list_head: Option<NonNull<FreePageNode>>,
}

impl AllocatorState {
    /// Checks if an allocation of `size` bytes aligned to
    /// `align` fits into any of page nodes.
    ///
    /// # Panics
    ///
    /// Panics in debug mode when `align` is not a power of two.
    fn is_allocatable(&self, size: usize, align: usize) -> bool {
        let mut current_node = self.list_head;
        while let Some(mut node_ptr) = current_node {
            // SAFETY: We have a valid allocation of `FreePageNode`.
            let node = unsafe { node_ptr.as_mut() };

            // Check if the frame is large enough to fit the whole allocation.
            let last_frame_addr = node_ptr.addr().get() + node.size - 1;
            let last_alloc_addr = align_up(node_ptr.addr().get(), align) + size - 1;
            if last_alloc_addr <= last_frame_addr {
                return true;
            }

            // Current frame is too small, advance to the next node.
            current_node = node.next;
        }

        false
    }

    /// Attempts to allocate `size` bytes at a given address in
    /// memory by inserting a new [`FreePageNode`].
    ///
    /// Returns [`Err`] when the list has no frame that would be
    /// large enough to fit the requested allocation.
    ///
    /// # Safety
    ///
    /// - `address` must have the alignment of [`FreePageNode`]
    ///   *and* `size` alignment.
    ///
    /// - `size` must be small enough to not overflow the address
    ///   space when added to any address in the spanned memory
    ///   region for allocation. Additionally, it must be a
    ///   multiple of [`core::mem::align_of`]`::<FreePageNode>()`.
    #[allow(clippy::result_unit_err)]
    unsafe fn try_allocate(&mut self, address: usize, size: usize) -> Result<(), ()> {
        let mut current_node = self.list_head;
        let mut previous_next = ptr::addr_of_mut!(self.list_head);

        while let Some(mut node) = current_node {
            // SAFETY: We have a valid allocation of `FreePageNode`.
            let mut current = node.as_mut();

            // Extract the address range covered by the current frame.
            let current_start_addr = node.addr().get();
            let current_last_addr = current_start_addr + current.size - 1;

            // Check if the range we want to allocate fits in the frame.
            if current_start_addr <= address && address + size - 1 <= current_last_addr {
                debug_assert!(is_aligned(address, align_of::<FreePageNode>()));

                // We are in bounds, so calculate the pointer to the next allocation node.
                let alloc = {
                    // SAFETY: `current_start_addr` is already non-null and
                    // address is guaranteed to be >= at this point.
                    let addr = NonZeroUsize::new_unchecked(address);
                    // SAFETY: `node` has provenance over the entire memory
                    // region, the resulting pointer is safe to dereference.
                    node.with_addr(addr)
                };

                // Do fragmentation at front, if necessary.
                if node != alloc {
                    previous_next = ptr::addr_of_mut!(current.next);

                    let alloc_node = alloc.as_uninit_mut().write(FreePageNode {
                        next: current.next,
                        size: current_start_addr + current.size - address,
                    });
                    *current = FreePageNode {
                        // SAFETY: `alloc`'s storage is initialized at this point.
                        // Its lifetime is tied to the whole allocation.
                        next: Some(alloc),
                        size: address - current_start_addr,
                    };

                    current = alloc_node;
                }

                // Do fragmentation at tail, if necessary.
                if current.size != size {
                    // SAFETY:
                    //
                    // * The caller guarantees that adding `size` to `addr`
                    //   does not overflow the address space.
                    //
                    // * The resulting pointer will get `alloc`'s provenance
                    //   over the whole allocated region, so it will be safe
                    //   to dereference and access.
                    let next =
                        alloc.map_addr(|addr| NonZeroUsize::new_unchecked(addr.get() + size));

                    next.as_uninit_mut().write(FreePageNode {
                        next: current.next,
                        size: current.size - size,
                    });
                    *current = FreePageNode {
                        // SAFETY: `next`'s storage is initialized at this point.
                        // Its lifetime is tied to the whole allocation.
                        next: Some(next),
                        size,
                    };

                    *previous_next = Some(next);
                } else {
                    *previous_next = current.next;
                }

                return Ok(());
            }

            // Advance to the next node in the list.
            previous_next = ptr::addr_of_mut!(current.next);
            current_node = current.next;
        }

        Err(())
    }

    /// Frees the allocation of `size` bytes at `ptr`.
    ///
    /// # Safety
    ///
    /// - `chunk` must point to a valid allocation at the same
    ///   address that was passed to `try_allocate` and meet
    ///   the other requirements listed there.
    ///
    /// - `size` must be the same size used for `try_allocate`
    ///   and meet the other requirements listed there.
    unsafe fn free(&mut self, mut chunk: NonNull<FreePageNode>, size: usize) {
        let mut previous_next = ptr::addr_of_mut!(self.list_head);

        if let Some(mut node) = self.list_head {
            // Calculate the address range of the allocated chunk.
            let chunk_start_addr = chunk.addr().get();
            let chunk_end_addr = chunk_start_addr + size;

            loop {
                // SAFETY: We assume a valid allocation of `FreePageNode`.
                let current = node.as_mut();

                // Calculate the address range of the current node.
                let current_start_addr = node.addr().get();
                let current_end_addr = current_start_addr + current.size;

                // Attempt to coalesce the chunk with existing nodes, where possible.
                if chunk_start_addr < chunk_end_addr {
                    if chunk_end_addr < current_start_addr {
                        *chunk.as_mut() = FreePageNode {
                            next: Some(node),
                            size,
                        };
                        break;
                    } else if chunk_end_addr == current_start_addr {
                        *chunk.as_mut() = FreePageNode {
                            next: current.next,
                            size: current.size + size,
                        };
                        break;
                    }
                } else if current_end_addr == chunk_start_addr {
                    current.size += size;
                    return;
                }

                // Advance to the next node in the list.
                previous_next = ptr::addr_of_mut!(current.next);
                if let Some(next) = current.next {
                    node = next;
                } else {
                    *chunk.as_mut() = FreePageNode { next: None, size };
                    current.next = Some(chunk);

                    return;
                }
            }
        } else {
            // We don't have a head node, so turn `chunk` into it.
            *chunk.as_mut() = FreePageNode { next: None, size };
        }

        // Link the previous next node to the chunk we just free'd.
        *previous_next = Some(chunk);
    }
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

/// An allocation done by the [`InitialPageAllocator`].
///
/// Chunks must be manually freed with [`InitialPageAllocator::free`]
/// when no longer used.
#[derive(Debug)]
#[must_use = "Chunks must be manually freed through the allocator"]
pub struct Chunk {
    start: PhysAddr,
    size: usize,
}

impl InitialPageAllocator {
    /// Creates a new page allocator with empty state.
    ///
    /// All attempts to allocate memory will fail until
    /// [`InitialPageAllocator::init`] was called.
    #[inline(always)]
    pub const fn new() -> Self {
        let null_addr = unsafe { PhysAddr::new_unchecked(ptr::null_mut::<u8>()) };

        Self {
            state: AllocatorState {
                start_address: null_addr,
                end_address: null_addr,
                list_head: None,
            },
        }
    }

    /// Creates a new page allocator with the given state.
    ///
    /// Depending on whether the state is already initialized
    /// or not, [`InitialPageAllocator::init`] might have to
    /// be called before the resulting object can be used.
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
    /// Panics in debug mode when the allocator is already
    /// initialized.
    ///
    /// # Safety
    ///
    /// `start` must be a non-null, physical memory address
    /// with MMU disabled.
    ///
    /// Further, it must be pointer-aligned and point to an
    /// unaliased, possibly uninitialized memory region with
    /// sufficient provenance for writing the region.
    pub unsafe fn init(&mut self, start: PhysAddr) {
        // Make sure we're not already initialized.
        debug_assert_eq!(self.state.start_address.addr(), 0);

        self.state.start_address = start;
        self.state.end_address = start;
    }

    /// Allocates `size` bytes at an aligned address.
    ///
    /// Returns the allocation [`Chunk`]. The contents of
    /// the allocation may or may not be initialized.
    ///
    /// Returns an error when the allocator is not already
    /// initialized with [`InitialPageAllocator::init`].
    ///
    /// # Panics
    ///
    /// Panics in debug mode when `align` is not a power of two
    /// or when `size` would overflow the address space.
    #[allow(clippy::result_unit_err)]
    pub fn allocate_aligned(&mut self, size: usize, align: usize) -> Result<Chunk, ()> {
        // When the allocator is not initialized, we opt out.
        if self.state.start_address.addr() == 0 {
            return Err(());
        }

        // Ensure that our free page list is non-empty.
        while !self.state.is_allocatable(size, align) {
            unsafe {
                // SAFETY: `end_address` is non-null when `start_address`
                // is. And we checked this precondition earlier.
                self.state.free(
                    NonNull::new_unchecked(self.state.end_address.as_mut_ptr::<FreePageNode>()),
                    u64::BITS as usize * BUILD_CONFIG.page_size,
                );

                self.state.end_address =
                    self.state.end_address.map_addr(|addr| addr + size).unwrap();
            }
        }

        // Allocate at a random, aligned address.
        let aligned_start = self.state.start_address.align_up(align).unwrap();
        //let aligned_end = self.state.end_address.align_down(align).unwrap();
        // TODO: Compute random offset for the allocation address.
        unsafe {
            self.state
                .try_allocate(aligned_start.addr(), size)
                .map(|()| Chunk {
                    start: aligned_start,
                    size,
                })
        }
    }

    /// Allocates `size` bytes at a `size`-aligned address.
    ///
    /// Returns the allocation [`Chunk`]. The contents of
    /// the allocation may or may not be initialized.
    ///
    /// Returns an error when the allocator is not already
    /// initialized with [`InitialPageAllocator::init`].
    ///
    /// # Panics
    ///
    /// Panics in debug mode when `align` is not a power of two
    /// or when `size` would overflow the address space.
    #[allow(clippy::result_unit_err)]
    pub fn allocate(&mut self, size: usize) -> Result<Chunk, ()> {
        self.allocate_aligned(size, size)
    }

    /// Consumes an allocated [`Chunk`], marking its memory as
    /// reusable.
    ///
    /// The allocation must not be used past the call to this
    /// method.
    pub fn free(&mut self, chunk: Chunk) {
        let Chunk { start, size } = chunk;
        unsafe {
            // SAFETY: `Chunks` can only be created by the allocator itself;
            // the pointer is guaranteed to be non-null, a valid allocation
            // and having its correct allocation size associated with it.
            let ptr = NonNull::new_unchecked(start.as_mut_ptr::<FreePageNode>());
            self.state.free(ptr, size)
        }
    }
}

impl Chunk {
    /// Gets the physical start address of the allocation.
    #[inline]
    pub fn addr(&self) -> PhysAddr {
        self.start
    }

    /// Gets the size of the allocation in bytes.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets an immutable slice that spans the allocated memory
    /// region.
    ///
    /// Its contents are potentially uninitialized.
    pub fn as_slice(&self) -> &[MaybeUninit<u8>] {
        // SAFETY: The `start` pointer has sufficient provenance for the
        // subset of the memory allocator region that is the allocation.
        unsafe {
            slice::from_raw_parts(
                self.start.as_ptr::<u8>().cast::<MaybeUninit<u8>>(),
                self.size,
            )
        }
    }

    /// Gets a mutable slice that spans the allocated memory
    /// region.
    ///
    /// Its contents are potentially uninitialized.
    pub fn as_slice_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        // SAFETY: The `start` pointer has sufficient provenance for the
        // subset of the memory allocator region that is the allocation.
        unsafe {
            slice::from_raw_parts_mut(
                self.start.as_mut_ptr::<u8>().cast::<MaybeUninit<u8>>(),
                self.size,
            )
        }
    }
}
