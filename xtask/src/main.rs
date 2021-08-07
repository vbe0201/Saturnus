use anyhow::anyhow;
use argh::FromArgs;
use xtask::package::Package;

/// 'xtask' is the build system used for builidng Saturnus and it's components.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    /// which package should be targetted by the action. only required for some actions
    #[argh(option, short = 'p')]
    package: Option<String>,
    /// run the action for every package
    #[argh(switch)]
    all: bool,
    #[argh(subcommand)]
    /// the action which should be performed
    cmd: Action,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    Run(RunConfig),
    Build(BuildConfig),
    Lint(LintConfig),
    Llvm(LlvmConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// build and run the provided package
struct RunConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
/// build the provided package
struct BuildConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "llvm")]
/// subcommand for invoking llvm bintools on the given package
struct LlvmConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
    /// which tool should be invoked
    #[argh(positional)]
    tool: String,
    /// the arguments which will be given to the llvm tool
    #[argh(positional)]
    rest: Vec<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lint")]
/// run clippy and rustfmt on the package
struct LintConfig {
    /// pass `--check` argument to rustfmt
    #[argh(switch)]
    check: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Arguments = argh::from_env();

    if args.all {
        for pkg in xtask::package::all_packages() {
            execute_action(&args, pkg)?;
        }
    } else {
        let pkg = args
            .package
            .as_ref()
            .ok_or_else(|| anyhow!("'package' option required for this action"))
            .and_then(|name| {
                xtask::package::find_package(name)
                    .ok_or_else(|| anyhow!("'{}' is an unknown package", name))
            });

        execute_action(&args, pkg?)?;
    }

    Ok(())
}

fn execute_action(args: &Arguments, pkg: Package) -> anyhow::Result<()> {
    match args.cmd {
        Action::Build(ref cfg) => {
            let elf = xtask::build_package(cfg.release, cfg.bsp.as_deref(), pkg)?;
            xtask::extract_binary(elf)?;
        }
        Action::Run(ref cfg) => {
            let elf = xtask::build_package(cfg.release, cfg.bsp.as_deref(), pkg)?;
            let path = xtask::extract_binary(elf)?;
            xtask::runner::run_qemu_aarch64(path)?;
        }
        Action::Llvm(ref cfg) => {
            xtask::run_llvm_tool(cfg.release, cfg.bsp.as_deref(), pkg, &cfg.tool, &cfg.rest)?;
        }
        Action::Lint(ref cfg) => {
            xtask::lint(cfg.check, pkg)?;
        }
    }

    Ok(())
}
