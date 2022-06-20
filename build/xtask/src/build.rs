//! Implementation of the build system action `build`.

use std::{io::BufReader, path::PathBuf};

use anyhow::{anyhow, Result};
use xshell::{cmd, Shell};

use crate::{
    package::{self, Package},
    rustc,
    target::Target,
};

/// Builds the full Saturnus kernel image and returns the path
/// to it.
///
/// The resulting binary will include both the Kernel, the Kernel
/// Loader and all the Kernel Initial Processes (KIPs).
pub fn build_kernel(sh: &Shell, target: &Target, release: bool) -> Result<PathBuf> {
    let kernel_loader = build(sh, &package::KERNEL_LOADER, target, release)?;
    let kernel = build(sh, &package::KERNEL, target, release)?;

    let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse()?;
    let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse()?;
    let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse()?;

    let image_path = {
        let mut root = rustc::project_root();
        root.push("target");
        root.push("dist");

        root = sh.create_dir(root)?;
        root.push("kernel.bin");

        root
    };

    // TODO: Add support for baking in KIPs.
    kernel_image::ImageBuilder::default()
        .with_page_size(target.config.page_size as usize)
        .with_loader(kernel_loader)?
        .with_kernel(kernel)?
        .with_version(version_major, version_minor, version_patch)
        .finalize(&image_path)?;

    Ok(image_path)
}

fn build(sh: &Shell, pkg: &Package, target: &Target, release: bool) -> Result<PathBuf> {
    let _cwd = sh.push_dir(rustc::project_root());

    let release_arg = if release { &["--release"][..] } else { &[] };
    let cargo_name = pkg.cargo_name;
    let target_json = target.target_json;
    let features = &["--no-default-features", "--features", target.board];

    // Before we start, copy the requested linker script over.
    // This is done to establish a standard target jsons can refer to.
    let linker_script = format!("build/linker-scripts/{}-{}.ld", target.arch, pkg.name);
    sh.copy_file(linker_script, "link.ld")?;

    // Build the requested package using cargo.
    let output = cmd!(
        sh,
        "cargo build
            {release_arg...}
            -p {cargo_name}
            --target {target_json}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins
            --message-format=json-render-diagnostics"
    )
    .output()?;

    // Now that we're done, remove the previously copied linker script.
    sh.remove_path("link.ld")?;

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
