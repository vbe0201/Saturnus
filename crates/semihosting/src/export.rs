//! Implementation details used by macros.

use core::{
    arch::asm,
    fmt::{self, Write},
};

use crate::host::HostStream;

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

#[allow(clippy::result_unit_err)]
pub fn hstdout_str(s: &str) -> Result<(), ()> {
    unsafe {
        interrupt_free(|| {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(HostStream::stdout()?);
            }

            HSTDOUT
                .as_mut()
                .unwrap_unchecked()
                .write_str(s)
                .map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        interrupt_free(|| {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(HostStream::stdout()?);
            }

            HSTDOUT
                .as_mut()
                .unwrap_unchecked()
                .write_fmt(args)
                .map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_str(s: &str) -> Result<(), ()> {
    unsafe {
        interrupt_free(|| {
            if HSTDERR.is_none() {
                HSTDERR = Some(HostStream::stderr()?);
            }

            HSTDERR
                .as_mut()
                .unwrap_unchecked()
                .write_str(s)
                .map_err(drop)
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        interrupt_free(|| {
            if HSTDERR.is_none() {
                HSTDERR = Some(HostStream::stderr()?);
            }

            HSTDERR
                .as_mut()
                .unwrap_unchecked()
                .write_fmt(args)
                .map_err(drop)
        })
    }
}

#[inline(always)]
unsafe fn interrupt_free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Back up the current interrupt state from DAIF.
    let daif_i: u64;
    asm!(
        "mrs {state}, daif",
        "ubfx {state}, {state}, #7, #1",
        state = out(reg) daif_i,
        options(nostack)
    );

    // Disable interrupts.
    asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));

    // Run the user-supplied closure in the critical section.
    let result = f();

    // Restore the previously saved interrupt state.
    asm!(
        "mrs {tmp}, daif",
        "bfi {tmp}, {state}, #7, #1",
        "msr daif, {tmp}",
        tmp = out(reg) _,
        state = in(reg) daif_i,
        options(nostack),
    );

    result
}
