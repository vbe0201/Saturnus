//! IMPLEMENTATION DETAILS USED BY MACROS.

use core::fmt::{self, Write};

use libkern::irq::ScopedInterruptDisable;

use crate::host::HostStream;

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

fn interrupt_free<R>(f: impl FnOnce() -> R) -> R {
    let _irq_guard = unsafe { ScopedInterruptDisable::start() };
    f()
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
