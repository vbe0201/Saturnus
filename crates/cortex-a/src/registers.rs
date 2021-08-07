//! Abstractions for accessing [ARM system registers](https://developer.arm.com/documentation/den0024/a/ARMv8-Registers/System-registers).
//!
//! This module is based on [`tock-registers`](https://docs.rs/tock-registers).
//! Have a look at their documentation for usage on how to use it.

#[macro_use]
mod macros;

mod current_el;
mod daif;
mod dit;
mod elr_el;
mod fpcr;
mod fpsr;
mod nzvc;

pub use current_el::CurrentEL;
pub use daif::DAIF;
pub use dit::DIT;
pub use elr_el::{el1::ELR_EL1, el2::ELR_EL2, el3::ELR_EL3};
pub use fpcr::FPCR;
pub use fpsr::FPSR;
pub use nzvc::NZVC;
