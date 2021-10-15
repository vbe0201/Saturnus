//! Abstractions for page tables and other paging related structures.

pub mod addr;
pub use addr::{PhysAddr, VirtAddr};

pub mod page;
pub use page::{Page, PhysFrame};

pub mod granule;

mod page_table;
pub use page_table::PageTable;

mod error;
use core::ptr::NonNull;

use bitflags::bitflags;
pub use error::*;
use page::{PageSize, SupportedPageSize};

// TODO: Do a full cleanup.

/// A trait that is able to allocate physical page frames with a static size.
///
/// The APIs are resistant against misuse in that they only allow statically
/// validated and known page sizes. [`PhysAddr`]s are used to mark the starting
/// address of pages in memory.
///
/// # Safety
///
/// Valid [`PhysAddr`]s representing the start addresses of physical page frames
/// in memory must be returned. This implies correct alignment of the address.
///
/// The following `SIZE` bytes from that address may not be implicitly modified
/// until a user explicitly frees the frame.
pub unsafe trait PageAllocator {
    /// Tries to allocate a single new page frame of `SIZE` bytes.
    ///
    /// On success the start address to the frame is returned, [`None`]
    /// on failure.
    ///
    /// The allocated memory region may or may not have its contents
    /// initialized and the user is responsible for correctly interacting
    /// with it.
    fn allocate<const SIZE: usize>(&self) -> Option<PhysAddr>
    where
        page::PageSize<SIZE>: page::SupportedPageSize;

    /// Frees an allocated frame of `SIZE` bytes given its physical starting
    /// address in memory.
    ///
    /// # Safety
    ///
    /// This method is wildly unsafe and will trigger UB if `addr` and `SIZE`
    /// are not a matching pair from the [`PageAllocator::allocate`] operation
    /// of that same allocator.
    ///
    /// Same goes if a frame has already been deallocated prior to calling this
    /// function again.
    unsafe fn free<const SIZE: usize>(&self, addr: PhysAddr)
    where
        page::PageSize<SIZE>: page::SupportedPageSize;
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

bitflags! {
    /// Possible flags for a page table entry that points to a block / page in virtual memory.
    #[repr(transparent)]
    pub struct PageFlags: u64 {
        /// The Execute-never or Unprivileged execute-never field.
        const UXN = 1 << 54;
    }
}
