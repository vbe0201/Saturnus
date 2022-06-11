use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::Shell;
use xtask::{
    package::{all_packages, Package},
    target::*,
};

#[derive(Parser)]
#[clap(long_about = None)]
struct Cli {
    /// A supported Saturnus target to build for.
    #[clap(long, short, parse(try_from_str=parse_target))]
    target: Target,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Attempts to build the full kernel image.
    Build {
        /// Invokes cargo in release mode.
        #[clap(short, long)]
        release: bool,
    },

    /// Builds the kernel binaries to check for warnings/errors.
    Check {
        #[clap(parse(try_from_str=parse_package))]
        package: Package,

        /// Invokes cargo in release mode.
        #[clap(short, long)]
        release: bool,
    },

    /// Attempts to build then run the full kernel image.
    Run {
        /// Invokes cargo in release mode.
        #[clap(short, long)]
        release: bool,
    },

    /// Runs clippy and rustfmt on the whole project.
    Lint {
        /// Pass the `--check` flag to rustfmt.
        #[clap(short, long)]
        check: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let shell = Shell::new()?;
    match cli.action {
        Action::Build { release } => {
            let kernel = xtask::build::build_kernel(&shell, &cli.target, release)?;
            shell.copy_file(kernel, xtask::rustc::project_root())?;
            Ok(())
        }

        Action::Check { package, release } => {
            xtask::check::check(&shell, &package, &cli.target, release)
        }

        Action::Run { release } => xtask::run::run(&shell, &cli.target, release),

        Action::Lint { check } => {
            all_packages().try_for_each(|p| xtask::lint::lint(&shell, p, &cli.target, check))
        }
    }
}

fn parse_target(target: &str) -> Result<Target, String> {
    all_targets()
        .find(|t| t.name.eq_ignore_ascii_case(target))
        .copied()
        .ok_or_else(|| "target is unsupported!".into())
}

fn parse_package(package: &str) -> Result<Package, String> {
    all_packages()
        .find(|p| p.name.eq_ignore_ascii_case(package))
        .copied()
        .ok_or_else(|| "package does not exist!".into())
}
