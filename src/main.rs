#![allow(clippy::too_many_arguments)]

mod commands;

use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
    /// Show current version
    #[arg(short, long)]
    version: bool,

    /// Show archive metadata
    #[arg(short, long)]
    metadata: bool,

    /// Input type
    #[arg(short = 't', long = "type")]
    input_type: Option<String>,

    /// Path to input file (if creating, multiple files can be separated by semicolons)
    #[arg(short, long)]
    input: Option<String>,

    /// Path to output file or directory
    #[arg(short, long)]
    output: Option<String>,

    /// List files in archive
    #[arg(short, long)]
    list: bool,

    /// Extract files from archive
    #[arg(short = 'x', long = "extract")]
    extract: bool,

    /// Create archive
    #[arg(short, long)]
    create: bool,

    /// by path
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    path: Option<String>,

    /// by index (faster than by path)
    #[arg(short = '1', long = "index", value_name = "INDEX")]
    index: Option<u32>,

    /// "all" (combine with -x or similar)
    #[arg(short, long)]
    all: bool,

    /// Skip integrity checks (not recommended, faster)
    #[arg(long, default_value = "false")]
    skip_integrity_checks: bool,

    /// Buffer size for file operations in bytes
    #[arg(long, default_value = "16777216")]
    buffer: u64,
}

fn get_command(args: &Args) -> &'static str {
    if args.version {
        "version"
    } else if args.metadata {
        "metadata"
    } else if args.list {
        "list"
    } else if args.extract {
        "extract"
    } else if args.create {
        "create"
    } else {
        ""
    }
}

fn main() {
    let args = Args::parse();

    let command = get_command(&args);

    match command {
        "version" => commands::version::version(),
        "metadata" => commands::metadata::metadata(
            args.input_type.unwrap(),
            args.input.unwrap(),
            !args.skip_integrity_checks,
            args.buffer,
        ),
        "list" => commands::list::list(
            args.input_type.unwrap(),
            args.input.unwrap(),
            !args.skip_integrity_checks,
            args.buffer,
        ),
        "extract" => commands::extract::extract(
            args.input_type.unwrap(),
            args.input.unwrap(),
            args.output.unwrap(),
            args.index,
            args.path,
            args.all,
            !args.skip_integrity_checks,
            args.buffer,
        ),
        "create" => commands::create::create(
            args.input_type.unwrap(),
            args.input.unwrap(),
            args.output.unwrap(),
            args.buffer,
        ),
        _ => println!("Nothing to do, try --help"),
    }
}
