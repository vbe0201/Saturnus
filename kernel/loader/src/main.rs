#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions, option_get_or_insert_default)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate saturnus_semihosting;

#[macro_use]
extern crate static_assertions;

#[macro_use]
mod macros;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/main.rs"]
mod arch_main;

mod bsp;
mod exception;
mod loader;
mod page_allocator;
mod paging;
mod panic;
mod rt;

use loader::KernelMap;
use page_allocator::PageAllocator;

/// The global page allocator that is used throughout the loader's runtime
/// for allocating pages.
pub(crate) static INITAL_PAGE_ALLOCATOR: PageAllocator = PageAllocator::new();

pub use arch_main::main;
