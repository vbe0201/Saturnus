//! Abstractions for page tables and other paging related structures.

pub mod addr;
pub use addr::{align_down, align_up, PhysAddr, VirtAddr};

pub mod page;
pub use page::{Page, PhysFrame};

pub mod granule;

mod page_table;
pub use page_table::PageTable;

mod error;
pub use error::*;

use bitflags::bitflags;
use core::ptr::NonNull;
use page::{PageSize, SupportedPageSize};

/// A trait that is able to allocate physical frames with a statically known size.
///
/// # Safety
///
/// Memory blocks returned from an allocator must return physical addresses to blocks of `SIZE`
/// bytes and the block must live until the block is freed
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
