//! Abstractions for page tables and other paging related structures.

pub mod addr;
pub use addr::{align_down, align_up, PhysAddr, VirtAddr};

pub mod page;
pub use page::{Page, PhysFrame};

use core::ptr::NonNull;
use page::{PageSize, SupportedPageSize};

/// A trait that is able to allocate physical frames with the page size specified
/// by the generic argument.
///
/// # Safety
///
/// Memory blocks returned from an allocator must return physical addresses to blocks of `SIZE`
/// bytes and the block must live until the block is freed
pub unsafe trait FrameAllocator<const SIZE: usize>
where
    PageSize<SIZE>: SupportedPageSize,
{
    /// Tries to allocate a single physical frame.
    ///
    /// The returned block may or may not have it's contents initialized.
    /// `None` is returned if the allocation failed.
    fn allocate(&self) -> Option<NonNull<[u8; SIZE]>>;

    /// Deallocates the physical frame referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// `ptr` must be an currently allocated frame of **this** allocator instance.
    unsafe fn deallocate(&self, ptr: NonNull<u8>);
}

unsafe impl<A, const SIZE: usize> FrameAllocator<SIZE> for &A
where
    PageSize<SIZE>: SupportedPageSize,
    A: FrameAllocator<SIZE> + ?Sized,
{
    fn allocate(&self) -> Option<NonNull<[u8; SIZE]>> {
        (**self).allocate()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        unsafe { (**self).deallocate(ptr) }
    }
}
