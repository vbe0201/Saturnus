//! IMPLEMENTATION DETAILS USED BY MACROS.

use core::fmt::{self, Write};

use libkern::irq::without_interrupts;

use crate::host::HostStream;

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

#[allow(clippy::result_unit_err)]
pub fn hstdout_str(s: &str) -> Result<(), ()> {
    unsafe {
        without_interrupts(|| {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(HostStream::stdout()?);
            }

            HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        without_interrupts(|| {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(HostStream::stdout()?);
            }

            HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_str(s: &str) -> Result<(), ()> {
    unsafe {
        without_interrupts(|| {
            if HSTDERR.is_none() {
                HSTDERR = Some(HostStream::stderr()?);
            }

            HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        without_interrupts(|| {
            if HSTDERR.is_none() {
                HSTDERR = Some(HostStream::stderr()?);
            }

            HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
        })
    }
}
