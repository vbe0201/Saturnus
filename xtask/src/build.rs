//! Implementation of the `build` action in the build system.

use std::path::PathBuf;

use anyhow::Result;
use xshell::cmd;

use crate::{package::Package, rustc};

/// Generates a raw binary file from the given ELF executable using
/// `llvm-objcopy`.
pub fn generate_raw_binary(elf: PathBuf) -> Result<PathBuf> {
    let mut output = elf.clone();
    output.set_extension("bin");

    // call llvm-objcopy to generate a raw binary from the ELF file
    let objcopy = rustc::llvm_tool("objcopy")?;
    cmd!("{objcopy} -S -O binary {elf} {output}").run()?;

    Ok(output)
}

/// Builds a given package in the Saturnus source tree for a specific board.
pub fn build(pkg: Package, bsp: Option<&str>, release: bool) -> Result<PathBuf> {
    let _cwd = xshell::pushd(rustc::root_dir())?;

    // build the kernel using cargo
    let release_arg = if release { &["--release"][..] } else { &[] };
    let Package {
        cargo_name, target, ..
    } = pkg;

    let features = match bsp {
        Some(bsp) => vec![
            "--no-default-features".to_owned(),
            "--features".to_owned(),
            format!("bsp-{}", bsp),
        ],
        None => vec![],
    };

    cmd!(
        "cargo build
            {release_arg...}
            -p {cargo_name}
            --target {target}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins
            --message-format=json-render-diagnostics"
    )
    .run()?;

    // FIXME: Use proper way to get the target directory (parse cargo output using cargo_metadata)
    let target_dir = match release {
        true => "target/aarch64-saturnus-none/release",
        false => "target/aarch64-saturnus-none/debug",
    };

    Ok(PathBuf::from(format!("{}/{}", target_dir, cargo_name)))
}
