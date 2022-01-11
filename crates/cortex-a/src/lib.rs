//! A library providing low-level hardware access to Cortex-A processors.
//!
//! This is intended as a temporary, more feature-rich replacement of the
//! [`cortex-a`] crate until we get to upstream parts of this.
//!
//! [`cortex-a`]: https://crates.io/crates/cortex-a

#![no_std]
#![feature(allocator_api, generic_const_exprs)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
#![allow(incomplete_features, unreachable_patterns)]

#[macro_use]
extern crate static_assertions;

pub mod asm;
pub mod paging;
pub mod registers;
