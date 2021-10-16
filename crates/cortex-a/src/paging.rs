//! Abstractions for page tables and other paging related structures.

use core::ptr::NonNull;

use bitflags::bitflags;
use libutils::assert::{Assert, True, False};

pub mod addr;
pub use self::addr::{PhysAddr, VirtAddr};

pub mod page;
pub use self::page::{Page, PhysFrame};

pub mod granule;

mod page_table;
pub use self::page_table::PageTable;

mod error;
pub use self::error::*;

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
pub unsafe trait PageAllocator<const PAGE_SIZE: usize>
where
    page::PageSize<PAGE_SIZE>: page::SupportedPageSize,
{
    /// Tries to allocate one or more new page frames of `SIZE` bytes
    /// in total.
    ///
    /// On success the start address to the (subsequent) frames is returned,
    /// [`None`] on failure.
    ///
    /// The allocated memory region may or may not have its contents
    /// initialized and the user is responsible for correctly interacting
    /// with it.
    fn allocate<const SIZE: usize>(&mut self) -> Option<PhysAddr>
    where
        Assert::<{ SIZE % PAGE_SIZE == 0 }>: True;

    /// Frees one or more subsequently allocated frames of `SIZE` bytes in total
    /// given their physical starting address in memory.
    ///
    /// # Safety
    ///
    /// This method is wildly unsafe and will trigger UB if `addr` and `SIZE`
    /// are not a matching pair from the [`PageAllocator::allocate`] operation
    /// of that same allocator.
    ///
    /// Same goes if a frame has already been deallocated prior to calling this
    /// function again.
    unsafe fn free<const SIZE: usize>(&mut self, addr: PhysAddr)
    where
        Assert::<{ SIZE % PAGE_SIZE == 0 }>: True;
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
