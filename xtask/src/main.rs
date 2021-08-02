use argh::FromArgs;

/// 'xtask' is the build system used for builidng Saturnus and it's components.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    #[argh(subcommand)]
    /// the action which should be performed
    cmd: Action,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    Run(RunConfig),
    Build(BuildConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// build and run the Saturnus Kernel in QEMU
struct RunConfig {
    /// build the kernel in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
/// build the Saturnus Kernel
struct BuildConfig {
    /// build the kernel in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Arguments = argh::from_env();

    match args.cmd {
        Action::Build(cfg) => {
            let elf = xtask::build(cfg.release)?;
            xtask::extract_binary(elf)?;
        }
        Action::Run(cfg) => {
            let elf = xtask::build(cfg.release)?;
            let path = xtask::extract_binary(elf)?;
            xtask::run(path)?;
        }
    }

    Ok(())
}
