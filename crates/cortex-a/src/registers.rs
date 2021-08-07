//! Abstractions for accessing [ARM system registers](https://developer.arm.com/documentation/den0024/a/ARMv8-Registers/System-registers).
//!
//! This module is based on [`tock-registers`](https://docs.rs/tock-registers).
//! Have a look at their documentation for usage on how to use it.
//!
//! **Note:** For every register module there's also a constant with the same name, which is
//! just hidden from the docs to avoid duplication of the names.

#[macro_use]
mod macros;

mod current_el;
mod daif;

pub use current_el::CurrentEL;
pub use daif::DAIF;
