#![no_std]
#![no_main]

#![deny(rustdoc::broken_intra_doc_links)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
