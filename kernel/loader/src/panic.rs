use core::panic::PanicInfo;

use saturnus_semihosting::debug::{self, EXIT_FAILURE};

/// The panic handler of the loader application.
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    // Print the panic information to HOST stderr.
    heprintln!("{}", info);

    // Exit the semihosting session.
    debug::exit(EXIT_FAILURE);

    // Halt in an infinite loop should a debugger require execution to continue.
    loop {}
}
