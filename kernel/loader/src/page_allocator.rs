use core::{
    mem::MaybeUninit,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

/// The size of a single memory page (4 KiB).
pub const PAGE_SIZE: usize = 0x1000;

/// The initial page allocator that the loader will use for page table operations.
///
/// This allocator is an extremely simple bump allocator as this is enough for
/// the loader page table operations.
pub struct PageAllocator {
    next_address: AtomicUsize,
}

impl PageAllocator {
    /// Create a new page allocator.
    pub const fn new() -> Self {
        Self {
            next_address: AtomicUsize::new(0),
        }
    }

    /// This sets the allocator's next address.
    pub unsafe fn initialize(&self, addr: usize) {
        self.next_address.store(addr, Ordering::Relaxed);
    }

    /// This just clears the allocator's next address thus, freeing all the memory that
    /// was allocated with thie allocator.
    pub unsafe fn finalize(&self) {
        self.next_address.store(0, Ordering::Relaxed);
    }

    /// Allocates a single page of memory.
    pub fn allocate(&self) -> NonNull<[MaybeUninit<u8>; PAGE_SIZE]> {
        let page = self.next_address.fetch_add(PAGE_SIZE, Ordering::Relaxed);
        let page = page as *mut [MaybeUninit<u8>; PAGE_SIZE];
        let page = NonNull::new(page).expect("tried to allocate page but next_address is 0");
        page
    }
}
