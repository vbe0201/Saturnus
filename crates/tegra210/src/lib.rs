//! Low-level hardware access library for Tegra X1 SoCs.
//!
//! # Organization
//!
//! The crate consists of several namespaced module representing one complex
//! hardware block as described in the Tegra Reference Manual.
//!
//! Each of these modules expose an additional `raw` module which contains
//! raw bindings to all MMIO registers for unsafe but more flexible access
//! to the hardware.
//!
//! On top of that, safe abstractions over the raw registers to implement
//! common functionality may be provided where reasonable.
//!
//! Note that some of these abstractions may be highly specific for exclusive
//! use in Saturnus code and should not really be used on other platforms.
//! Those will be marked as such in their documentation.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate static_assertions;

pub mod mc;
