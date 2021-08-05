//! IMPLEMENTATION DETAILS USED BY MACROS.

use core::fmt::{self, Write};

use crate::host::HostStream;

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

fn interrupt_free<R>(f: impl FnOnce() -> R) -> R {
    // Disable interrupts.
    let daif_old: u64;
    unsafe {
        llvm_asm!("mrs $0, daif" : "=r"(daif_old) ::: "volatile");
        llvm_asm!("msr daifset, #2" ::: "memory" : "volatile");
    }

    let ret = f();

    // Re-enable interrupts.
    let cur_daif: u64;
    unsafe {
        llvm_asm!("mrs $0, daif" : "=r"(cur_daif) ::: "volatile");
        llvm_asm!("msr daif, $0" :: "r"(((cur_daif & !0x80) as u32) | (daif_old & 0x80) as u32) :: "volatile");
    }

    ret
}

#[allow(clippy::result_unit_err)]
pub fn hstdout_str(s: &str) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream::stdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
    })
}

#[allow(clippy::result_unit_err)]
pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream::stdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
    })
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_str(s: &str) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(HostStream::stderr()?);
        }

        HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
    })
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(HostStream::stderr()?);
        }

        HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
    })
}
