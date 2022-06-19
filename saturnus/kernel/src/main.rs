#![feature(optimize_attribute)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;

#[inline(never)]
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
