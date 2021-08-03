//! Implementation of the operations xtask can perform (build, run, etc).

pub mod package;
pub mod runner;
pub mod rustc;

use anyhow::Result;
use package::Package;
use std::path::PathBuf;
use xshell::cmd;

/// Build the Saturnus kernel and return the path to the ELF file which was produced by cargo.
pub fn build_package(release: bool, pkg: Package) -> Result<PathBuf> {
    let _cwd = xshell::pushd(rustc::root_dir());

    // build the kernel using cargo
    let release_arg = if release { &["--release"][..] } else { &[] };
    let Package {
        cargo_name, target, ..
    } = pkg;

    cmd!(
        "cargo build
            {release_arg...}
            -p {cargo_name}
            --target {target}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    // FIXME: Use proper way to get the target directory (parse cargo output using cargo_metadata)
    let target_dir = match release {
        true => "target/aarch64-saturnus-none/release",
        false => "target/aarch64-saturnus-none/debug",
    };

    Ok(PathBuf::from(format!("{}/{}", target_dir, cargo_name)))
}

/// Extract the raw binary file from the given ELF file using llvm-objcopy.
pub fn extract_binary(elf: PathBuf) -> Result<PathBuf> {
    // call llvm-objcopy to extract a raw binary from the ELF file
    let objcopy_bin = rustc::llvm_tool("objcopy")?;

    let mut output = elf.clone();
    output.set_extension("bin");

    cmd!("{objcopy_bin} -O binary {elf} {output}").run()?;

    Ok(output)
}

/// Run the given llvm tool on the produced kernel file with the given arguments.
pub fn run_llvm_tool(release: bool, pkg: Package, tool: &str, args: &[String]) -> Result<()> {
    let kernel = build_package(release, pkg)?;
    let tool_bin = rustc::llvm_tool(tool)?;
    cmd!("{tool_bin} {kernel} {args...}")
        .echo_cmd(false)
        .run()?;
    Ok(())
}

/// Lint and format the given package.
pub fn lint(check: bool, pkg: Package) -> Result<()> {
    let _cwd = xshell::pushd(rustc::root_dir());

    let Package {
        cargo_name, target, ..
    } = pkg;

    cmd!(
        "cargo clippy
            -p {cargo_name}
            --target {target}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    let check_arg = if check { &["--", "--check"][..] } else { &[] };
    cmd!(
        "cargo fmt
            -p {cargo_name} {check_arg...}"
    )
    .run()?;

    Ok(())
}
