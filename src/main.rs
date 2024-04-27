use clap::Parser;

use rustii::*;

fn main() {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Render { input_file_path, output, scale, contrast } => {
            if let Err(e) = render(input_file_path, output, scale, contrast) {
                handle_error(e, 1);
            }
        },

        Commands::Open { input_file_path } => {
            if let Err(e) = open(input_file_path) {
                handle_error(e, 2);
            }
        }
    };
}

