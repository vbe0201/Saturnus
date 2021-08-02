use argh::FromArgs;

/// 'xtask' is the build system used for builidng Saturnus and it's components.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    /// build the kernel in release mode (optimizations enabled)
    #[argh(switch)]
    release: bool,
    #[argh(subcommand)]
    /// the action which should be performed
    cmd: Action,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    Run(RunConfig),
    Build(BuildConfig),
    LLVM(LlvmConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// build and run the Saturnus Kernel in QEMU
struct RunConfig {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
/// build the Saturnus Kernel
struct BuildConfig {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "llvm")]
/// subcommand for accessing the shipped llvm binutil tools
struct LlvmConfig {
    /// which tool should be invoked
    #[argh(positional)]
    tool: String,
    /// the arguments which will be given to the llvm tool
    #[argh(positional)]
    rest: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Arguments = argh::from_env();

    match args.cmd {
        Action::Build(..) => {
            let elf = xtask::build(args.release)?;
            xtask::extract_binary(elf)?;
        }
        Action::Run(..) => {
            let elf = xtask::build(args.release)?;
            let path = xtask::extract_binary(elf)?;
            xtask::run(path)?;
        }
        Action::LLVM(cfg) => {
            xtask::run_llvm_tool(args.release, &cfg.tool, cfg.rest)?;
        }
    }

    Ok(())
}
