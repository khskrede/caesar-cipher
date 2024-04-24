use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encrypt input data using key
    Encrypt {
        key: i64,
        input_file_path: Option<String>,
        output_file_path: Option<String>,
    },

    /// Decrypt input data using key
    Decrypt {
        key: i64,
        input_file_path: Option<String>,
        output_file_path: Option<String>,
    },
}

#[derive(Args)]
struct Arguments {
    rotate_by: i64,

    #[clap(short, long)]
    input_file_path: Option<String>,

    #[clap(short, long)]
    output_file_path: Option<String>,
}

pub struct ApplicationConfig {
    pub rotate_by: i64,
    pub input_file_path: Option<String>,
    pub output_file_path: Option<String>,
}

impl ApplicationConfig {
    pub fn from_args() -> ApplicationConfig {
        let parsed_args = Cli::parse();
        let (rotate_by, input_file_path, output_file_path) = match parsed_args.command {
            Command::Encrypt {
                key,
                input_file_path,
                output_file_path,
            } => (key, input_file_path, output_file_path),
            Command::Decrypt {
                key,
                input_file_path,
                output_file_path,
            } => (-key, input_file_path, output_file_path),
        };

        ApplicationConfig {
            rotate_by,
            input_file_path,
            output_file_path,
        }
    }
}
