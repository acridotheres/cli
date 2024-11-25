mod extract;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract an archive
    #[clap(visible_alias("x"), visible_alias("ex"), visible_alias("unpack"))]
    Extract {
        /// Path to the archive
        #[arg(short = 'i', long)]
        file: String,

        /// Path to the output directory
        #[arg(short, long)]
        output: Option<String>,

        /// Format type of the archive, if not provided it will be auto-detected
        #[arg(short, long)]
        format: Option<String>,

        /// Buffer size for reading the archive
        #[arg(short, long, default_value = "4096")]
        buffer_size: u64,

        /// Method to use for autodetection (extension, magic, parse)
        #[arg(short = 'm', long, default_value = "extension")]
        autodetection_method: String,

        /// Password for the archive
        #[arg(short = 'P', long)]
        password: Option<String>,
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
                autodetection_method,
                password,
            } => {
                extract::run(
                    file,
                    output,
                    format,
                    buffer_size,
                    autodetection_method,
                    password,
                );
            }
        }
    }
}
