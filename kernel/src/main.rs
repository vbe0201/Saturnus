#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
