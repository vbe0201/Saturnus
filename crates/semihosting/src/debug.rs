//! Interacting with debugging agent.
//!
//! # Example
//!
//! How to terminate a QEMU session. The program should be
//! running under QEMU with semihosting enabled (use
//! `-semihosting` flag).
//!
//! ```no_run
//! # use saturnus_semihosting::debug::{self, EXIT_FAILURE, EXIT_SUCCESS};
//!
//! if 2 + 2 == 4 {
//!     // Report success.
//!     debug::exit(EXIT_SUCCESS);
//! } else {
//!     // Report failure.
//!     debug::exit(EXIT_FAILURE);
//! }
//! ```

/// Values taken from Section 5.5.2 of ADS Debug Target Guide
/// (DUI0058).
#[allow(missing_docs)]
pub enum Exception {
    // Hardware reason codes.
    BranchThroughZero = 0x20000,
    UndefinedInstr = 0x20001,
    SoftwareInterrupt = 0x20002,
    PrefetchAbort = 0x20003,
    DataAbort = 0x20004,
    AddressException = 0x20005,
    IRQ = 0x20006,
    FIQ = 0x20007,

    // Software reason codes.
    BreakPoint = 0x20020,
    WatchPoint = 0x20021,
    StepComplete = 0x20022,
    RunTimeErrorUnknown = 0x20023,
    InternalError = 0x20024,
    UserInterruption = 0x20025,
    ApplicationExit = 0x20026,
    StackOverflow = 0x20027,
    DivisionByZero = 0x20028,
    OSSpecific = 0x20029,
}

/// Status enum for `exit` syscall.
pub type ExitStatus = Result<(), ()>;

/// Successful execution of a program.
pub const EXIT_SUCCESS: ExitStatus = Ok(());

/// Unsuccessful execution of a program.
pub const EXIT_FAILURE: ExitStatus = Err(());

/// Reports to the debugger that the execution has completed.
///
/// This call can be used to terminate the QEMU session and
/// report back success or failure. If you need to pass more
/// than one type of error, consider using [`report_exception`]
/// syscall instead.
///
/// This call should not return. However ,it is possible for the
/// debugger to request that the application continues. In this
/// case, the function will normally return.
pub fn exit(status: ExitStatus) {
    match status {
        EXIT_SUCCESS => report_exception(Exception::ApplicationExit),
        EXIT_FAILURE => report_exception(Exception::RunTimeErrorUnknown),
    }
}

/// Reports an exception reason code to the debugger directly.
///
/// Exception handlers can use this SWI at the end of handler
/// chains as the default action to indicate that the exception
/// has not been handled.
///
/// This call should not return. However, it is possible for the
/// debugger to request that the application continues. In this
/// case, the function will normally return.
pub fn report_exception(reason: Exception) {
    unsafe {
        syscall1!(REPORT_EXCEPTION, reason);
    }
}
