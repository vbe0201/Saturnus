use core::panic::PanicInfo;

/// The panic handler of the loader application.
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    // TODO
    loop {}
}
