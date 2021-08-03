use std::{env, fs, io, path::PathBuf};

const LINKER_SCRIPT: &str = "../linker-scripts/kernel.ld";

fn main() -> io::Result<()> {
    // copy the kernel linker script into the output directory so the linker can find it
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(LINKER_SCRIPT, out.join("link.ld"))?;

    // tell rustc to search in the out directory for linker stuff
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", LINKER_SCRIPT);
    Ok(())
}
