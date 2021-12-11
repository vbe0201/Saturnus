//! Interfaces to the functions exposed to Secure Monitor Calls from EL3.

use saturnus_smc::{ctx::SecureMonitorContext, smc, SUPERVISOR_ID};

use crate::irq::ScopedInterruptDisable;

#[path = "smc/rng.rs"]
mod rng;
pub use self::rng::*;

/// Calls a privileged Secure Monitor function with `ctx` while disabling all
/// interrupts for the duration of the call.
///
/// # Safety
///
/// This is hardware land. Use at your own discretion.
#[inline(never)]
pub unsafe fn call_privileged_secure_monitor_function(ctx: &mut SecureMonitorContext) {
    // Disable interrupts for the scope of the call.
    let _irq_guard = ScopedInterruptDisable::start();

    // Perform the Secure Monitor call.
    smc::<SUPERVISOR_ID>(ctx);
}
