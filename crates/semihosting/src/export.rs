//! IMPLEMENTATION DETAILS USED BY MACROS.

use core::fmt::{self, Write};

use crate::host::HostStream;

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

fn interrupt_free<R>(f: impl FnOnce() -> R) -> R {
    let cpsr_old: u32;
    unsafe {
        llvm_asm!("mrs $0, cpsr" : "=r"(cpsr_old) ::: "volatile");
        llvm_asm!("cpsid i" :::: "volatile");
    }

    let ret = f();

    if cpsr_old & 0x80 == 0 {
        unsafe {
            llvm_asm!("cpsie i" :::: "volatile");
        }
    }

    ret
}

pub fn hstdout_str(s: &str) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream::stdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
    })
}

pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream::stdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
    })
}

pub fn hstderr_str(s: &str) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(HostStream::stderr()?);
        }

        HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
    })
}

pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    interrupt_free(|| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(HostStream::stderr()?);
        }

        HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
    })
}
