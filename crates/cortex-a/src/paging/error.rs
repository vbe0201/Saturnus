/// Errors that can happen while mapping virtual pages.
#[derive(Debug)]
pub enum MapError {
    /// Failed to allocate a new page from the underlying allocator.
    PageAllocationFailed,
    /// The virtual address was already mapped.
    PageAlreadyMapped,
}
