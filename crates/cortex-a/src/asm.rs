//! Wrappers around common ARMv8-A instructions.

pub mod barrier;

/// The classic no-operation instruction.
#[inline(always)]
pub fn nop() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("nop", options(nostack, nomem)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Wait For Interrupt
///
/// For more details on wfi, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub unsafe fn wfi() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("wfi", options(nostack, nomem)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Wait For Event
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub unsafe fn wfe() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("wfe", options(nostack, nomem)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Send EVent.Locally
///
/// SEV causes an event to be signaled to the local core within a multiprocessor system.
///
/// For more details of wfe - sev/sevl pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn sevl() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("sevl", options(nostack, nomem)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Send EVent.
///
/// SEV causes an event to be signaled to all cores within a multiprocessor system.
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn sev() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("sev", options(nostack, nomem)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
