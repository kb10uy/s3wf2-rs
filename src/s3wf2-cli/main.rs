mod sub_debug;
mod sub_format;
mod util;

use anyhow::Result;
use clap::Clap;

/// Standalone tool for ShortStoryServer Writer's Format v2
#[derive(Debug, Clap)]
#[clap(version, author)]
struct Arguments {
    /// Specifies subcommand
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Clap)]
enum Subcommand {
    /// Parses source text and converts into another format
    Format(FormatArguments),

    /// Parses source text and shows the result in debug format
    Debug(DebugArguments),
}

#[derive(Debug, Clap)]
struct FormatArguments {
    /// Source text file (will be stdin when omitted)
    input: Option<String>,

    /// Output file (- for stdout)
    #[clap(short, long)]
    output: Option<String>,

    /// Format type
    #[clap(short = 't', long = "type")]
    format_type: Option<String>,

    /// Shows verbose debug information
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Clap)]
struct DebugArguments {
    /// Source text file (will be stdin when omitted)
    input: Option<String>,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    match args.subcommand {
        Subcommand::Format(args) => sub_format::subcommand_format(args),
        Subcommand::Debug(args) => sub_debug::subcommand_debug(args),
    }
}
