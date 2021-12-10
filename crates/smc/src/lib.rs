//! Implementation of the ARM Secure Monitor Calling Convention with respect
//! to the extensions made by Nintendo.
//!
//! The heart of this crate is the [`smc`] function which conveniently allows
//! triggering SMCs with pre-built state and retrieving results from it.
//!
//! Refer to [here] for an in-depth description of the original calling
//! convention.
//!
//! [here]: https://documentation-service.arm.com/static/5f8ea482f86e16515cdbe3c6?token=

#![no_std]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]
#![feature(asm)]

#[macro_use]
extern crate static_assertions;

pub mod call;
pub mod ctx;
pub mod result;
pub mod service;

/// ID for [`smc`]s triggered from user level.
pub const USER_ID: usize = 0;
/// ID for [`smc`]s triggered from supervisor level.
pub const SUPERVISOR_ID: usize = 1;

/// Triggers a Secure Monitor Call with an ID denoted by `ID`.
///
/// The state provided by `ctx` will be loaded as input and overwritten
/// by the invoked SMC handler.
///
/// # Safety
///
/// This is hardware land. Use at your own discretion.
#[allow(unsafe_op_in_unsafe_fn)]
#[inline(always)]
pub unsafe fn smc<const ID: usize>(ctx: &mut ctx::SecureMonitorContext) {
    // Perform the SMC with inputs from `ctx` where we also store results.
    // We also back up and restore the current thread pointer value in `x18`
    // which may be clobbered across the call.
    asm!(
        "mov {tmp}, x18",
        "smc {id}",
        "mov x18, {tmp}",
        tmp = out(reg) _,
        id = const ID,
        inlateout("w0") ctx.x[0],
        inlateout("x1") ctx.x[1],
        inlateout("x2") ctx.x[2],
        inlateout("x3") ctx.x[3],
        inlateout("x4") ctx.x[4],
        inlateout("x5") ctx.x[5],
        inlateout("x6") ctx.x[6],
        inlateout("x7") ctx.x[7],
        lateout("x8") _,
        lateout("x9") _,
        lateout("x10") _,
        lateout("x11") _,
        lateout("x12") _,
        lateout("x13") _,
        lateout("x14") _,
        lateout("x15") _,
        lateout("x16") _,
        lateout("x17") _,
    )
}
