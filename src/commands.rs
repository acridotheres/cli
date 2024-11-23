mod extract;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract an archive
    #[clap(visible_alias("x"), visible_alias("ex"), visible_alias("unpack"))]
    Extract {
        #[arg(short = 'i', long)]
        file: String,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long)]
        format: Option<String>,

        #[arg(short, long, default_value = "4096")]
        buffer_size: u64,
    },
}

impl Command {
    pub fn run(self) {
        use Command::*;
        match self {
            Extract {
                file,
                output,
                format,
                buffer_size,
            } => {
                extract::run(file, output, format, buffer_size);
            }
        }
    }
}
