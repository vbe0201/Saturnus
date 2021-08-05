//! Semihosting for ARM Cortex-A processors.
//!
//! This is heavily inspired by the [`cortex-m-semihosting`] crate, but mainly intended to
//! be used in the specific environment Saturnus is tested in. Therefore, many features we
//! did not consider necessary to cover have been erased in this crate.
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on an ARM target to communicate
//! and use the Input/Output facilities on a host computer that is running a debugger."
//!
//! - ARM
//!
//! # Interface
//!
//! This crate provides implementations of [`core::fmt::Write`], in conjunction with
//! [`core::format_args!`] or the [`core::write!`] macro for user-friendly construction and
//! printing of formatted strings.
//!
//! Since semihosting operations are modeled as [system calls][sc], this crate exposes an
//! untyped `syscall!` interface just like the [`sc`] crate does.
//!
//! # Forewarning
//!
//! Semihosting operations are *very* slow. Like, each WRITE operation can take hundreds of
//! milliseconds.
//!
//! # Reference
//!
//! For documentation about the semihosting operations, check:
//!
//! `Chapter 8 - Semihosting` of the ['ARM Compiler toolchain Version 5.0'][pdf] manual.
//!
//! [`cortex-m-semihosting`]: https://github.com/rust-embedded/cortex-m-semihosting
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//! [pdf]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0471e/DUI0471E_developing_for_arm_processors.pdf

#![no_std]
#![feature(llvm_asm)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
mod macros;

pub mod debug;
#[doc(hidden)]
pub mod export;
pub mod host;
pub mod ops;

/// Performs a semihosting operation, takes a pointer to an argument block.
///
/// # Safety
///
/// - `nr` must be a valid syscall from [`crate::ops`].
/// - `arg` must point to a valid argument block for the syscall.
#[inline(always)]
pub unsafe fn syscall<T>(nr: usize, arg: &T) -> usize {
    unsafe { syscall1(nr, arg as *const T as usize) }
}

/// Performs a semihosting operation, takes one integer as an argument.
///
/// # Safety
///
/// - `nr` must be a valid syscall from [`crate::ops`].
/// - `arg` must be an address that points to a valid argument block.
#[inline(always)]
pub unsafe fn syscall1(mut nr: usize, arg: usize) -> usize {
    llvm_asm!("HLT #0xF000" : "+{x0}"(nr) : "{x1}"(arg) : "memory", "lr" : "volatile");
    nr
}
