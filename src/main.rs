use std::process;

use clap::Parser;

use rustii::APP_NAME;

fn main() {
    let cli = rustii::Cli::parse();

    match &cli.command {
        rustii::Commands::Render { input_file_path, output_file_path, scale } => {
            match rustii::render(&input_file_path, &output_file_path, &scale) {
                Ok(_) => (),
                Err(e) => handle_error(e)
            };
        },

        rustii::Commands::Play { input_file_path } => {
            match rustii::play(&input_file_path) {
                Ok(_) => (),
                Err(e) => handle_error(e)
            };
        }
    };
}

fn handle_error(error: &str) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(1);
}
