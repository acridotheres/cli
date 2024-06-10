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
}

fn get_command(args: &Args) -> &'static str {
    if args.version {
        "version"
    } else {
        ""
    }
}

fn main() {
    let args = Args::parse();

    let command = get_command(&args);

    if !args.json {
        match command {
            "version" => text::get_ver(),
            _ => println!("Nothing to do, try --help")
        }
    } else {
        match command {
            "version" => json::get_ver(),
            _ => print!("{{}}\n")
        }
    }
}
