use anyhow::anyhow;
use argh::FromArgs;
use xtask::package::Package;

/// Arguments to `xtask`, the build system of the Saturnus project.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    /// which package should be targeted by the action. only require
    /// for some actions
    #[argh(option, short = 'p')]
    package: Option<String>,

    /// run the action for every package, not only one
    #[argh(switch)]
    all: bool,

    #[argh(subcommand)]
    /// the action which should be performed
    cmd: Action,
}

/// Concrete actions to be performed by the build system.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    Run(RunConfig),
    Build(BuildConfig),
    Check(CheckConfig),
    Lint(LintConfig),
    Llvm(LlvmConfig),
}

/// build and run the provided package
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
struct RunConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,

    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
}

/// build the provided package
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
struct BuildConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,

    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
}

/// check the provided package
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "check")]
struct CheckConfig {
    /// build the package in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,

    /// specifies for which board to build the package (e.g. QEMU, Switch, etc)
    #[argh(option)]
    bsp: Option<String>,
}

/// subcommand for invoking llvm bintools on the given package
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "llvm")]
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

/// run clippy and rustfmt on the package
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lint")]
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
            let elf = xtask::build::build(pkg, cfg.bsp.as_deref(), cfg.release)?;
            xtask::build::generate_raw_binary(elf)?;
        }
        Action::Check(ref cfg) => {
            xtask::check::check(pkg, cfg.bsp.as_deref(), cfg.release)?;
        }
        Action::Run(ref cfg) => {
            let elf = xtask::build::build(pkg, cfg.bsp.as_deref(), cfg.release)?;
            let raw = xtask::build::generate_raw_binary(elf)?;
            xtask::run::run(raw)?;
        }
        Action::Llvm(ref cfg) => {
            xtask::llvm::llvm(pkg, cfg.bsp.as_deref(), cfg.release, &cfg.tool, &cfg.rest)?;
        }
        Action::Lint(ref cfg) => {
            xtask::lint::lint(pkg, cfg.check)?;
        }
    }

    Ok(())
}
