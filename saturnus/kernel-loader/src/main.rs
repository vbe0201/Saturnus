#![feature(naked_functions, strict_provenance)]
#![no_std]
#![no_main]

mod arch;
mod panic;
mod reloc;

#[no_mangle]
extern "C" fn main() {
    todo!()
}
