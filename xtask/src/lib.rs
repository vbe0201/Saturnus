//! Implementation of the operations xtask can perform (build, run, etc).

use anyhow::Result;
use std::path::PathBuf;
use xshell::cmd;

/// Run the given binary file in QEMU.
pub fn run(binary: PathBuf) -> Result<()> {
    cmd!(
        "qemu-system-aarch64
            -cpu cortex-a57
            -machine virt
            -nographic
            -semihosting-config enable=on,target=native
            -kernel {binary}"
    )
    .run()?;
    Ok(())
}

/// Build the Saturnus kernel and return the path to the ELF file which was produced by cargo.
pub fn build(release: bool) -> Result<PathBuf> {
    let _cwd = xshell::pushd(root());

    // build the kernel using cargo
    let release_arg = if release { &["--release"][..] } else { &[] };
    cmd!(
        "cargo build
            {release_arg...}
            -p kernel
            --target targets/aarch64-saturnus-none.json
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    // FIXME: Use proper way to get the target directory (parse cargo output using cargo_metadata)
    let target_dir = match release {
        true => "target/aarch64-saturnus-none/release",
        false => "target/aarch64-saturnus-none/debug",
    };

    Ok(PathBuf::from(format!("{}/kernel", target_dir)))
}

/// Extract the raw binary file from the given ELF file using llvm-objcopy.
pub fn extract_binary(elf: PathBuf) -> Result<PathBuf> {
    // call llvm-objcopy to extract a raw binary from the ELF file
    let objcopy_bin = llvm_tool("objcopy")?;

    let mut output = elf.clone();
    output.set_extension("bin");

    cmd!("{objcopy_bin} -O binary {elf} {output}").run()?;

    Ok(output)
}

/// Returns the path to the root of the workspace.
fn root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path
}

/// Get the path to the sysroot of the current rust compiler.
fn sysroot() -> Result<String> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = cmd!("{rustc} --print sysroot").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

/// Get the path to the llvm tool with the given name, that is shipped with rust.
fn llvm_tool(tool: &str) -> Result<PathBuf> {
    let sysroot = sysroot()?;
    let mut pathbuf = PathBuf::from(sysroot);
    pathbuf.push("lib");
    pathbuf.push("rustlib");
    pathbuf.push(rustc_version::version_meta()?.host);
    pathbuf.push("bin");
    pathbuf.push(format!("llvm-{}", tool));
    Ok(pathbuf)
}
