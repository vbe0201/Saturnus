//! Helpers and utilities used throughout the whole project.

#![no_std]
#![allow(incomplete_features)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
#![feature(const_transmute_copy, generic_const_exprs)]

pub mod assert;
pub mod bits;
pub mod mem;
