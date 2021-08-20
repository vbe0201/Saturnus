//! A library providing low-level hardware access to Cortex-A processors.
//!
//! This is intended as a temporary, more feature-rich replacement of the
//! [`cortex-a`] crate until we get to upstream parts of this.
//!
//! [`cortex-a`]: https://crates.io/crates/cortex-a

#![no_std]
#![feature(asm, const_panic, allocator_api)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
// required to not warn about inline assembly constructs
#![allow(unreachable_patterns)]

pub mod asm;
pub mod paging;
pub mod registers;
