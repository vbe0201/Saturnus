use anyhow::anyhow;
use argh::FromArgs;

/// 'xtask' is the build system used for builidng Saturnus and it's components.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    /// which package should be targetted by the action. only required for some actions
    #[argh(option, short = 'p')]
    package: Option<String>,
    #[argh(subcommand)]
    /// the action which should be performed
    cmd: Action,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    Run(RunConfig),
    Build(BuildConfig),
    Llvm(LlvmConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// build and run the provided package
struct RunConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
/// build the provided package
struct BuildConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "llvm")]
/// subcommand for invoking llvm bintools on the given package
struct LlvmConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
    /// which tool should be invoked
    #[argh(positional)]
    tool: String,
    /// the arguments which will be given to the llvm tool
    #[argh(positional)]
    rest: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Arguments = argh::from_env();

    let pkg = args
        .package
        .ok_or(anyhow!("'package' option required for this action"))
        .and_then(|name| {
            xtask::package::find_package(&name)
                .ok_or_else(|| anyhow!("'{}' is an unknown package", name))
        });

    match args.cmd {
        Action::Build(BuildConfig { release }) => {
            let elf = xtask::build_package(release, pkg?)?;
            xtask::extract_binary(elf)?;
        }
        Action::Run(RunConfig { release }) => {
            let elf = xtask::build_package(release, pkg?)?;
            let path = xtask::extract_binary(elf)?;
            xtask::runner::run_qemu_aarch64(path)?;
        }
        Action::Llvm(cfg) => {
            xtask::run_llvm_tool(cfg.release, pkg?, &cfg.tool, cfg.rest)?;
        }
    }

    Ok(())
}
