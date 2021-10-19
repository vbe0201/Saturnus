//! Implementations of the Secure Monitor Calls featured by Horizon's Secure
//! Monitor which lives in EL3.

use crate::irq::ScopedInterruptDisable;

#[derive(Clone, Debug, Default)]
#[repr(C)]
struct SecureMonitorArguments {
    x: [u64; 8],
}

assert_eq_size!(SecureMonitorArguments, [u64; 8]);

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn call_privileged_secure_monitor_function(args: &mut SecureMonitorArguments) {
    // Disable interrupts for the scope of the call.
    let _irq_guard = ScopedInterruptDisable::start();

    // Perform the SMC with all registers as inputs where we also store the results.
    asm!(
        "smc #1",
        inlateout("x0") args.x[0],
        inlateout("x1") args.x[1],
        inlateout("x2") args.x[2],
        inlateout("x3") args.x[3],
        inlateout("x4") args.x[4],
        inlateout("x5") args.x[5],
        inlateout("x6") args.x[6],
        inlateout("x7") args.x[7],
        options(nostack, nomem),
    )
}

/// SMCs used throughout early kernel bootstrap.
pub mod init {
    /// Generates random bytes using the Secure Monitor's access to the Tegra
    /// Security Engine's CPRNG.
    pub fn generate_random_bytes<T>() -> Result<T, ()> {
        todo!()
    }
}
