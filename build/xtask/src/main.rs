use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(long_about = None)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Attempts to build the full kernel image.
    Build {
        /// A supported Saturnus target to build for.
        target: Option<String>,

        /// Invokes cargo in release mode.
        #[clap(short, long)]
        release: bool,
    },

    /// Builds the kernel binaries to check for warnings/errors.
    Check {
        /// A supported Saturnus target to build for.
        target: Option<String>,

        /// Invokes cargo in release mode.
        #[clap(short, long)]
        release: bool,
    },

    /// Attempts to build then run the full kernel image.
    Run {
        /// A supported Saturnus target to build for.
        target: Option<String>,

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

fn main() {
    let cli = Cli::parse();
    todo!()
}
