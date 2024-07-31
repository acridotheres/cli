mod json;
mod text;

use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
    /// Show current version
    #[arg(short, long)]
    version: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Show archive metadata
    #[arg(short, long)]
    metadata: bool,

    /// Input type
    #[arg(short = 't', long = "type")]
    input_type: Option<String>,

    /// Path to input file
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

    /// by path
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    path: Option<String>,

    /// by index (faster than by path)
    #[arg(short = '1', long = "index", value_name = "INDEX")]
    index: Option<u32>,

    /// "all" (combine with -x or similar)
    #[arg(short, long)]
    all: bool,
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
    } else {
        ""
    }
}

fn main() {
    let args = Args::parse();

    let command = get_command(&args);

    let format = args.input_type.unwrap_or_else(|| {
        // add format detection here
        "".to_string()
    });
    let format = format.as_str();

    if !args.json {
        match command {
            "version" => text::get_version(),
            "metadata" => match format {
                "zip" => text::zip_metadata(args.input.unwrap()),
                _ => println!("Unknown format"),
            },
            "list" => match format {
                "zip" => text::zip_list(args.input.unwrap()),
                _ => println!("Unknown format"),
            },
            "extract" => match format {
                "zip" => text::zip_extract(
                    args.input.unwrap(),
                    args.output.unwrap(),
                    args.index,
                    args.path,
                    args.all,
                ),
                _ => println!("Unknown format"),
            },
            _ => println!("Nothing to do, try --help"),
        }
    } else {
        match command {
            "version" => json::get_version(),
            _ => print!("{{}}\n"),
        }
    }
}
