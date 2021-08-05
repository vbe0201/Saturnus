//! Host I/O operations.

use core::{fmt, slice};

use crate::ops;

/// A byte stream to host, e.g. host's stdout or stderr.
#[derive(Clone, Copy)]
pub struct HostStream {
    fd: usize,
}

impl HostStream {
    /// Attempts to construct a new handle to the host's standard output and returns it.
    pub fn stdout() -> Result<Self, ()> {
        open(":tt\0", ops::open::W_TRUNC)
    }

    /// Attempts to construct a new handle to the host's standard error and returns it.
    pub fn stderr() -> Result<Self, ()> {
        // There is actually no stderr access in ARM Semihosting documentation.
        // Use convention used in libgloss:
        // https://sourceware.org/git/?p=newlib-cygwin.git;a=blob;f=libgloss/arm/syscalls.c#l176
        open(":tt\0", ops::open::W_APPEND)
    }

    /// Attempts to write the entire `buffer` into this sink.
    pub fn write_all(&mut self, buffer: &[u8]) -> Result<(), ()> {
        write_all(self.fd, buffer)
    }
}

impl fmt::Write for HostStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

fn open(name: &str, mode: usize) -> Result<HostStream, ()> {
    let name = name.as_bytes();
    match unsafe { syscall!(OPEN, name.as_ptr(), mode, name.len() - 1) } as isize {
        -1 => Err(()),
        fd => Ok(HostStream { fd: fd as usize }),
    }
}

fn write_all(fd: usize, mut buffer: &[u8]) -> Result<(), ()> {
    while !buffer.is_empty() {
        match unsafe { syscall!(WRITE, fd, buffer.as_ptr(), buffer.len()) } {
            // Done
            0 => return Ok(()),

            // `n` bytes were not written.
            n if n <= buffer.len() => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe { slice::from_raw_parts(buffer.as_ptr().offset(offset), n) }
            }

            // Error
            _ => return Err(()),
        }
    }

    Ok(())
}
