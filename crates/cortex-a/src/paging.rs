//! Abstractions for page tables and other paging related structures.

use core::ptr::NonNull;

pub mod addr;
pub use self::addr::{PhysAddr, VirtAddr};

mod error;
pub use self::error::*;

pub mod granule;

pub mod page;
pub use self::page::{Page, PhysFrame};

//mod page_table;
//pub use self::page_table::PageTable;

pub mod table_entry;

// TODO: Do a full cleanup.

/// A trait that is able to allocate memory within physical page frames.
///
/// This means that the user can choose to allocate arbitrary quantities of
/// memory not necessarily restricted to full pages.
///
/// # Safety
///
/// Valid [`PhysAddr`]s representing the start addresses of physical page frames
/// in memory must be returned. Correct alignment of these addresses is assumed.
///
/// The following `size` bytes from that address may not be implicitly modified
/// until a user explicitly frees the allocation with [`PageAllocator::free`].
pub unsafe trait PageAllocator {
    /// The page size assumed by the allocator.
    const PAGE_SIZE: usize;

    /// Tries to allocate a contiguous memory region of `size` bytes.
    ///
    /// On success the start address to the allocated memory is returned,
    /// [`None`] on failure.
    ///
    /// The allocated memory region may or may not have its contents
    /// initialized and the user is responsible for correctly interacting
    /// with it.
    fn allocate(&mut self, size: usize) -> Option<PhysAddr>;

    /// Frees a contiguously allocated memory region of `size` bytes in total
    /// given the physical starting address in memory.
    ///
    /// # Safety
    ///
    /// This method is wildly unsafe and will trigger UB if `addr` and `size`
    /// are not a matching pair from the [`PageAllocator::allocate`] operation
    /// of that same allocator.
    ///
    /// Same goes if a frame has already been deallocated prior to calling this
    /// function again.
    unsafe fn free(&mut self, addr: PhysAddr, size: usize);
}

/// A trait that is able to allocate physical frames with a statically known size.
///
/// # Safety
///
/// Memory blocks returned from an allocator must return physical addresses to blocks of `SIZE`
/// bytes and the block must live until the block is freed
// TODO: Replace all occurences of `FrameAllocator` with `PageAllocator`.
pub unsafe trait FrameAllocator {
    /// Tries to allocate a single physical frame.
    ///
    /// The returned block may or may not have it's contents initialized.
    /// `None` is returned if the allocation failed.
    fn allocate<const SIZE: usize>(&self) -> Option<NonNull<[u8; SIZE]>>;

    /// Deallocates the physical frame referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// - `ptr` must be an currently allocated frame of **this** allocator instance.
    /// - the `SIZE` argument must be the same that was used to allocate `ptr`
    unsafe fn deallocate<const SIZE: usize>(&self, ptr: NonNull<u8>);
}
