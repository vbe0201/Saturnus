//! A library for unifying shared kernel code.
//!
//! Code in this library is either:
//!
//! * re-used between kernel and loader simultaneously
//!
//! * generic enough to be usable outside of the kernel itself.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

pub mod init;
