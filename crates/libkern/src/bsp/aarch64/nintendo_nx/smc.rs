//! Implementations of the Secure Monitor Calls featured by Horizon's Secure
//! Monitor which lives in EL3.

use core::{mem::size_of, ptr};

use static_assertions::assert_eq_size;

use crate::{irq::ScopedInterruptDisable, sync::SpinLock};

// TODO: Make SMCs board-specific for Nintendo Switch.

#[allow(dead_code)] // TODO: Remove this later.
mod result {
    pub const SMC_SUCCESS: u64 = 0;
    pub const SMC_UNIMPLEMENTED: u64 = 1;
    pub const SMC_INVALID_ARGUMENT: u64 = 2;
    pub const SMC_IN_PROGRESS: u64 = 3;
    pub const SMC_NO_ASYNC_OPERATION: u64 = 4;
    pub const SMC_INVALID_ASYNC_OPERATION: u64 = 5;
    pub const SMC_NOT_PERMITTED: u64 = 6;
}

#[derive(Clone, Debug)]
#[repr(C)]
struct SecureMonitorArguments {
    x: [u64; 8],
}

assert_eq_size!(SecureMonitorArguments, [u64; 8]);

#[derive(Debug)]
#[repr(u32)]
enum Function {
    GenerateRandomBytes = 0xc3000005,
}

#[allow(unsafe_op_in_unsafe_fn)]
#[inline(never)]
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

/// Generates random bytes using the Secure Monitor.
///
/// This overrides the contents of the entire `buf` slice with the newly
/// generated data.
///
/// # Panics
///
/// Panics when `buf` exceeds a length of `0x38` bytes in total, which is
/// a conceptual limit for how many random bytes can be generated.
pub fn generate_random_bytes(buf: &mut [u8]) {
    static CRITICAL_SECTION: SpinLock<()> = SpinLock::new(());

    // This Secure Monitor call takes the size of bytes to generate in `x1`
    // and then overwrites `x1`-`x7` with quad words of random bytes. We
    // need to make sure that the user is not requesting a number of bytes
    // that we don't actually have the capacity to store inside the output.
    assert!(buf.len() <= size_of::<SecureMonitorArguments>() - size_of::<u64>());

    // Prepare the arguments for a call to `GetRandomBytes`.
    let mut args = unsafe { libutils::mem::zeroed::<SecureMonitorArguments>() };
    args.x[0] = Function::GenerateRandomBytes as u64;
    args.x[1] = buf.len() as u64;

    // Call the Secure Monitor.
    unsafe {
        let _irq_guard = ScopedInterruptDisable::start();
        let _section_token = CRITICAL_SECTION.lock();

        call_privileged_secure_monitor_function(&mut args);
    }

    // Make sure that the SMC was successful and trigger a kernel panic otherwise.
    assert_eq!(args.x[0], result::SMC_SUCCESS);

    // Copy the resulting bytes from the output pointer to `buf`.
    unsafe {
        // SAFETY: The result from the SMC was validated and we can thereby
        // assume that we got a valid pointer at `size` bytes for us to copy.
        ptr::copy_nonoverlapping(
            args.x[1..].as_ptr() as *const u8,
            buf.as_mut_ptr(),
            buf.len(),
        );
    }
}

/// SMCs used throughout early kernel bootstrap.
pub mod init {
    use core::{mem::size_of, ptr};

    use super::{result, Function, SecureMonitorArguments};

    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline(always)]
    unsafe fn call_privileged_secure_monitor_function(args: &mut SecureMonitorArguments) {
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

    /// Generates random bytes using the Secure Monitor.
    ///
    /// This overrides the contents of the entire `buf` slice with the newly
    /// generated data.
    ///
    /// # Panics
    ///
    /// Panics when `buf` exceeds a length of `0x38` bytes in total, which is
    /// a conceptual limit for how many random bytes can be generated.
    pub fn generate_random_bytes(buf: &mut [u8]) {
        // This Secure Monitor call takes the size of bytes to generate in `x1`
        // and then overwrites `x1`-`x7` with quad words of random bytes. We
        // need to make sure that the user is not requesting a number of bytes
        // that we don't actually have the capacity to store inside the output.
        assert!(buf.len() <= size_of::<SecureMonitorArguments>() - size_of::<u64>());

        // Prepare the arguments for a call to `GetRandomBytes`.
        let mut args = unsafe { libutils::mem::zeroed::<SecureMonitorArguments>() };
        args.x[0] = Function::GenerateRandomBytes as u64;
        args.x[1] = buf.len() as u64;

        // Call the Secure Monitor.
        unsafe {
            call_privileged_secure_monitor_function(&mut args);
        }

        // Make sure that the SMC was successful and trigger a kernel panic otherwise.
        assert_eq!(args.x[0], result::SMC_SUCCESS);

        // Copy the resulting bytes from the output pointer to `buf`.
        unsafe {
            // SAFETY: The result from the SMC was validated and we can thereby
            // assume that we got a valid pointer at `size` bytes for us to copy.
            ptr::copy_nonoverlapping(
                args.x[1..].as_ptr() as *const u8,
                buf.as_mut_ptr(),
                buf.len(),
            );
        }
    }
}
