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
#[macro_use]
macro_rules! syscall1 {
    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::ops::$nr, $a1 as usize)
    };
}
