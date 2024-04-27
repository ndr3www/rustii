use clap::Parser;

use rustii::{Cli, Commands, handle_error, open, render};

fn main() {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Render { input_file_path, output, scale, contrast } => {
            match render(input_file_path, output, scale, contrast) {
                Ok(()) => (),
                Err(e) => handle_error(e, 1)
            };
        },

        Commands::Open { input_file_path } => {
            match open(input_file_path) {
                Ok(()) => (),
                Err(e) => handle_error(e, 2)
            };
        }
    };
}

