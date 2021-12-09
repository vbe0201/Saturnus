//!

#![no_std]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate static_assertions;

pub mod call;
pub mod ctx;
pub mod result;
pub mod service;
