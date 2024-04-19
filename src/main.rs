use clap::Parser;

use rustii::*;

fn main() {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Render { input_file_path, output, scale, contrast } => {
            match render(input_file_path, output, scale, contrast) {
                Ok(_) => (),
                Err(e) => handle_error(e, 1)
            };
        },

        Commands::Open { input_file_path } => {
            match open(input_file_path) {
                Ok(_) => (),
                Err(e) => handle_error(e, 2)
            };
        }
    };
}

