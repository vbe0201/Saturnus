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

/// Macro version of [`crate::syscall1`].
#[macro_export]
macro_rules! syscall1 {
    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::ops::$nr, $a1 as usize)
    };
}

/// Macro for printing to the HOST standard output.
///
/// This is similar to the `print!` macro in the standard library.
/// Both panic on any failure to print.
#[macro_export]
macro_rules! hprint {
    ($s:expr) => {
        $crate::export::hstdout_str($s)
            .expect("failed to print via semihosting")
    };

    ($($tt:tt)*) => {
        $crate::export::hstdout_fmt(::core::format_args!($($tt)*))
            .expect("failed to print via semihosting")
    };
}

/// Macro for printing to the HOST standard output, with a newline.
///
/// This is similar to the `println!` macro in the standard library.
/// Both panic on any failure to print.
#[macro_export]
macro_rules! hprintln {
    () => {
        $crate::export::hstdout_str("\n")
            .expect("failed to print via semihosting")
    };

    ($s:expr) => {
        $crate::export::hstdout_str(::core::concat!($s, "\n"))
            .expect("failed to print via semihosting")
    };

    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstdout_fmt(::core::format_args!(::core::concat!($s, "\n"), $($tt)*))
            .expect("failed to print via semihosting")
    };
}

/// Macro for printing to HOST standard error.
///
/// This is similar to the `eprint!` macro in the standard library.
/// Both panic on any failure to print.
#[macro_export]
macro_rules! heprint {
    ($s:expr) => {
        $crate::export::hstderr_str($s)
            .expect("failed to print via semihosting")
    };

    ($($tt:tt)*) => {
        $crate::export::hstderr_fmt(::core::format_args!($($tt)*))
            .expect("failed to print via semihosting")
    };
}

/// Macro for printing to HOST standard error, with a newline.
///
/// This is similar to the `eprintln!` macro in the standard library.
/// Both panic on any failure to print.
#[macro_export]
macro_rules! heprintln {
    () => {
        $crate::export::hstderr_str("\n")
            .expect("failed to print via semihosting")
    };

    ($s:expr) => {
        $crate::export::hstderr_str(::core::concat!($s, "\n"))
            .expect("failed to print via semihosting")
    };

    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstderr_fmt(::core::format_args!(::core::concat!($s, "\n"), $($tt)*))
            .expect("failed to print via semihosting")
    };
}

/// Macro that prints and returns the value of a given expression
/// for quick and dirty debugging.
///
/// Works exactly like `dbg!` in the standard library, replacing
/// `eprintln!` with [`heprintln!`].
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::heprintln!("[{}:{}]", ::core::file!(), ::core::line!())
    };

    ($val:expr) => {
        // Use of `match` here is intentional because it affects
        // the lifetime of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::heprintln!(
                    "[{}:{}] {} = {:#?}",
                    ::core::file!(),
                    ::core::line!(),
                    ::core::stringify!($val),
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
