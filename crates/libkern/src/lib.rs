//! A library for unifying shared kernel code.
//!
//! Code in this library is either:
//!
//! * re-used between kernel and loader simultaneously
//!
//! * generic enough to be usable outside of the kernel itself.

#![no_std]
#![allow(incomplete_features, unreachable_patterns)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(const_fn_trait_bound, const_mut_refs, generic_const_exprs)]

pub mod bsp;
pub mod critical_section;
pub mod init;
pub mod irq;
pub mod spin;
pub mod system_control;
