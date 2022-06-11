//! Implementation of the build system action `build`.

use std::{io::BufReader, path::PathBuf};

use anyhow::{anyhow, Result};
use xshell::{cmd, Shell};

use crate::{package::Package, rustc, target::Target};

/// Builds the full Saturnus kernel image and returns the path
/// to it.
///
/// The resulting binary will include both the Kernel, the Kernel
/// Loader and all the Kernel Initial Processes (KIPs).
pub fn build_kernel(sh: &Shell, target: &Target, release: bool) -> Result<PathBuf> {
    todo!()
}

fn build(sh: &Shell, pkg: &Package, target: &Target, release: bool) -> Result<PathBuf> {
    let _cwd = sh.push_dir(rustc::project_root());

    let release_arg = if release { &["--release"][..] } else { &[] };
    let cargo_name = pkg.cargo_name;
    let triple = target.llvm_triple;
    let features = &["--no-default-features", "--features", target.board];

    // Build the requested package using cargo.
    let output = cmd!(
        sh,
        "cargo build
            {release_arg...}
            -p {cargo_name}
            --target {triple}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins
            --message-format=json-render-diagnostics"
    )
    .output()?;

    // Try to extract the produced ELF binary for successful builds.
    let artifact_path = extract_build_artifact(&output.stdout)
        .ok_or_else(|| anyhow!("Build failed! Please run the `check` subcommand for details"))?;

    // Convert to raw binary and return the path to it.
    make_raw_binary(sh, artifact_path)
}

fn extract_build_artifact(rustc_output: &[u8]) -> Option<PathBuf> {
    use cargo_metadata::Message;

    Message::parse_stream(BufReader::new(rustc_output))
        .into_iter()
        .find_map(|msg| match msg {
            Ok(Message::CompilerArtifact(a)) if a.executable.is_some() => Some(a),
            _ => None,
        })
        .and_then(|a| a.executable.map(PathBuf::from))
}

fn make_raw_binary(sh: &Shell, elf: PathBuf) -> Result<PathBuf> {
    let mut output = elf.clone();
    output.set_extension("bin");

    let objcopy = rustc::llvm_binutil(sh, "objcopy")?;
    cmd!(sh, "{objcopy} -S -O binary {elf} {output}").run()?;

    Ok(output)
}
