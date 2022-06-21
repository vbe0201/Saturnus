#![feature(strict_provenance)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;
mod reloc;

#[inline(never)]
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
