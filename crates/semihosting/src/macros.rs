/// Variable argument version of [`crate::syscall`].
#[macro_export]
macro_rules! syscall {
    ($nr:ident) => {
        $crate::syscall1($crate::ops::$nr, 0)
    };

    ($nr:ident, $($arg:expr),+) => {
        $crate::syscall($crate::ops::$nr, &[$($arg as usize),+])
    };
}

/// Macro version of `syscall1`.
#[macro_export]
macro_rules! syscall1 {
    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::ops::$nr, $a1 as usize)
    };
}

/// Macro for printing to the HOST standard output.
///
/// This is similar to the `print!` macro in the standard library. Both will panic on
/// any failure to print.
#[macro_export]
macro_rules! hprint {
    ($s:expr) => {
        $crate::export::hstdout_str($s)
    };
    ($($tt:tt)*) => {
        $crate::export::hstdout_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the HOST standard output, with a newline.
///
/// This is similar to the `println!` macro in the standard library. Both will panic
/// on any failure to print.
#[macro_export]
macro_rules! hprintln {
    () => {
        $crate::export::hstdout_str("\n")
    };
    ($s:expr) => {
        $crate::export::hstdout_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstdout_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}

/// Macro for printing to HOST standard error.
///
/// This is similar to the `eprint!` macro in the standard library. Both will panic on
/// any failure to print.
#[macro_export]
macro_rules! heprint {
    ($s:expr) => {
        $crate::export::hstderr_str($s)
    };
    ($($tt:tt)*) => {
        $crate::export::hstderr_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to HOST standard error, with a newline.
///
/// This is similar to the `eprintln!` macro in the standard library. Both will panic
/// on any failure to print.
#[macro_export]
macro_rules! heprintln {
    () => {
        $crate::export::hstderr_str("\n")
    };
    ($s:expr) => {
        $crate::export::hstderr_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstderr_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}

/// Macro that prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// Works exactly like `dbg!` in the standard library, replacing `eprintln!` with
/// [`heprintln!`].
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::heprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::heprintln!(
                    "[{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored.
    ($val:expr,) => {
        $crate::dbg!($val)
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
