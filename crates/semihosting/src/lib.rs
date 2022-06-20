//! Semihosting for ARM Cortex-A processors.
//!
//! This is based on the [`cortex-m-semihosting`] crate, but
//! primarily intended to be used for Saturnus debugging.
//! Therefore, many features we did not consider necessary to
//! cover have been erased here.
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on an
//! ARM target to communicate and use the Input/Output facilities
//! on a host computer that is running a debugger."
//!
//! - ARM
//!
//! # Interface
//!
//! This crate provides implementations of [`core::fmt::Write`], in
//! conjunction with [`core::format_args!`] or the [`core::write!`]
//! macro for user-friendly construction and printing of formatted
//! strings.
//!
//! Since semihosting operations are modeled as [system calls][sc],
//! this crate exposes an untyped [`syscall!`] interface just like
//! the [`sc`] crate does.
//!
//! # Forewarning
//!
//! Semihosting operations are *very* slow. Like, each WRITE operation
//! can take hundreds of milliseconds.
//!
//! # Reference
//!
//! For documentation about the semihosting operations, check:
//!
//! `Chapter 8 - Semihosting` of the [`ARM Compiler toolchain Version 5.0`][pdf]
//! manual.
//!
//! [`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//! [pdf]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0471e/DUI0471E_developing_for_arm_processors.pdf

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(strict_provenance)]

use core::arch::asm;

#[macro_use]
mod macros;

pub mod debug;
#[doc(hidden)]
pub mod export;
pub mod host;
pub mod ops;

/// Performs a semihosting operation, takes a pointer to an
/// argument block.
///
/// # Safety
///
/// - `nr` must be a valid syscall from [`crate::ops`].
/// - `arg` must point to a valid argument block for the syscall.
#[inline(always)]
pub unsafe fn syscall<T>(nr: usize, arg: &T) -> usize {
    syscall1(nr, (arg as *const T).addr())
}

/// Performs a semihosting operation, takes one integer as an
/// argument.
///
/// # Safety
///
/// - `nr` must be a valid syscall from [`crate::ops`].
/// - `arg` must point to a valid argument block for the syscall.
#[inline(always)]
pub unsafe fn syscall1(mut nr: usize, arg: usize) -> usize {
    asm!(
        "hlt #0xF000",
        inout("x0") nr,
        in("x1") arg,
        options(nostack)
    );

    nr
}
