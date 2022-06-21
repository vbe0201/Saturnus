//! Implementation for Kernel Panic handlers for debug/release.

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    // When building for debug...
    #[cfg(debug_assertions)]
    {
        // ...print the panic information to HOST stderr when in QEMU.
        #[cfg(feature = "qemu")]
        semihosting::heprintln!("{}", info);
    }

    // Exit the semihosting session when in QEMU.
    #[cfg(feature = "qemu")]
    semihosting::debug::exit(semihosting::debug::EXIT_FAILURE);

    // Halt the loader in an infinite loop.
    // This is official release behavior and is also useful when
    // debuggers decide to ignore our shutdown requests in QEMU.
    loop {}
}
