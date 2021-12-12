//! Implementation of the `build` action in the build system.

use std::{io::BufReader, path::PathBuf};

use anyhow::{anyhow, Result};
use cargo_metadata::Message;
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

    let build_out = cmd!(
        "cargo build
            {release_arg...}
            -p {cargo_name}
            --target {target}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins
            --message-format=json-render-diagnostics"
    )
    .echo_cmd(false)
    .output()?;

    // Parse the build output to find the generated executable path.
    let mut target = None;
    for message in Message::parse_stream(BufReader::new(&build_out.stdout[..])) {
        match message.unwrap() {
            Message::CompilerArtifact(artifact) if artifact.executable.is_some() => {
                target = artifact.executable;
                break;
            }
            _ => continue,
        }
    }

    Ok(PathBuf::from(target.ok_or_else(|| {
        anyhow!("no executable binary produced by this build")
    })?))
}
