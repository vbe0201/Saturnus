//! A library for unifying shared kernel code.
//!
//! Code in this library is either:
//!
//! * re-used between kernel and loader simultaneously
//!
//! * generic enough to be usable outside of the kernel itself.

#![no_std]
#![allow(unreachable_patterns)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
#![feature(asm, const_fn_trait_bound, const_mut_refs)]

#[macro_use]
extern crate static_assertions;

pub mod init;
pub mod irq;
pub mod scoped_lock;
pub mod smc;
pub mod spin;
pub mod system_control;
