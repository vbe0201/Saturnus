//!

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
#[inline(always)]
pub unsafe fn smc<const ID: usize>(ctx: &mut ctx::SecureMonitorContext) {
    // TODO: Preserve thread pointer.

    // Perform the SMC with inputs from `ctx` where we also store results.
    unsafe {
        asm!(
            "smc {id}",
            id = const ID,
            inlateout("x0") ctx.x[0],
            inlateout("x1") ctx.x[1],
            inlateout("x2") ctx.x[2],
            inlateout("x3") ctx.x[3],
            inlateout("x4") ctx.x[4],
            inlateout("x5") ctx.x[5],
            inlateout("x6") ctx.x[6],
            inlateout("x7") ctx.x[7],
        )
    }
}
