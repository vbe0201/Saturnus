//! Machinery for the early kernel initialization work.

mod paging;
pub use self::paging::{InitialPageAllocator, InitialPageTable};
