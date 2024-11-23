#![allow(clippy::too_many_arguments)]

mod commands;

use clap::Parser;
use commands::Command;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(command) => {
            command.run();
        }
        None => {
            println!("Acridotheres CLI\n\nPass the --help argument for help.\n\nVisit https://acridotheres.com/cli for more information.");
        }
    };
}
