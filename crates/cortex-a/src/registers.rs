//! Abstractions for accessing [ARM system registers](https://developer.arm.com/documentation/den0024/a/ARMv8-Registers/System-registers).
//!
//! This module is based on [`tock-registers`](https://docs.rs/tock-registers).
//! Have a look at their documentation for usage on how to use it.

#[macro_use]
mod macros;

// ARM special purpose register
mod current_el;
mod daif;
mod dit;
mod elr_el;
mod fpcr;
mod fpsr;
mod nzvc;
mod sp_el;
mod spsel;

pub use current_el::CurrentEL;
pub use daif::DAIF;
pub use dit::DIT;
pub use elr_el::{el1::ELR_EL1, el2::ELR_EL2, el3::ELR_EL3};
pub use fpcr::FPCR;
pub use fpsr::FPSR;
pub use nzvc::NZVC;
pub use sp_el::{el0::SP_EL0, el1::SP_EL1, el2::SP_EL2, el3::SP_EL3};
pub use spsel::SPSel;

// General system control registers
pub mod mair_el;

mod esr_el;
mod far_el;
mod midr_el1;
mod sctlr_el;
mod spsr_el;
mod tcr_el;
mod tpidr_el;
mod ttbr_el;
mod vbar_el;

pub use esr_el::{
    el1::ESR_EL1, el2::ESR_EL2, el3::ESR_EL3, ESR as ESR_EL1, ESR as ESR_EL2, ESR as ESR_EL3,
};
pub use far_el::{el1::FAR_EL1, el2::FAR_EL2, el3::FAR_EL3};
pub use midr_el1::MIDR_EL1;
pub use sctlr_el::{
    el1::SCTLR_EL1, el2::SCTLR_EL2, el3::SCTLR_EL3, SCTLR as SCTLR_EL1, SCTLR as SCTLR_EL2,
    SCTLR as SCTLR_EL3,
};
pub use spsr_el::{
    el1::SPSR_EL1, el2::SPSR_EL2, el3::SPSR_EL3, SPSR as SPSR_EL1, SPSR as SPSR_EL2,
    SPSR as SPSR_EL3,
};
pub use tcr_el::{
    el1::TCR_EL1, el2::TCR_EL2, el3::TCR_EL3, TCR as TCR_EL1, TCR as TCR_EL2, TCR as TCR_EL3,
};
pub use tpidr_el::{el1::TPIDR_EL1, el2::TPIDR_EL2, el3::TPIDR_EL3};
pub use ttbr_el::{
    el1_0::TTBR0_EL1, el1_1::TTBR1_EL1, el2_0::TTBR0_EL2, el2_1::TTBR1_EL2, el3::TTBR0_EL3,
    TTBR as TTBR0_EL1, TTBR as TTBR0_EL2, TTBR as TTBR0_EL3, TTBR as TTBR1_EL1, TTBR as TTBR1_EL2,
};
pub use vbar_el::{el1::VBAR_EL1, el2::VBAR_EL2, el3::VBAR_EL3};
