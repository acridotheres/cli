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
    metadata: Option<String>,
}

fn get_command(args: &Args) -> &'static str {
    if args.version {
        "version"
    } else if args.metadata != None {
        "metadata"
    } else {
        ""
    }
}

fn main() {
    let args = Args::parse();

    let command = get_command(&args);

    if !args.json {
        match command {
            "version" => text::get_version(),
            "metadata" => text::zip_metadata(args.metadata.unwrap().as_str()),
            _ => println!("Nothing to do, try --help"),
        }
    } else {
        match command {
            "version" => json::get_version(),
            _ => print!("{{}}\n"),
        }
    }
}
