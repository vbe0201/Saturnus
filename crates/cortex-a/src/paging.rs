//! Abstractions for page tables and other paging related structures.

pub mod addr;
pub use addr::{align_down, align_up, PhysAddr, VirtAddr};
