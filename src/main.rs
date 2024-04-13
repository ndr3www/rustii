use clap::Parser;

fn main() {
    let cli = rustii::Cli::parse();

    match &cli.command {
        rustii::Commands::Render { input_file_path, output_file_path, scale } => {
            match rustii::render(&input_file_path, &output_file_path, &scale) {
                Ok(_) => (),
                Err(e) => rustii::handle_error(e, 1)
            };
        },

        rustii::Commands::Play { input_file_path } => {
            match rustii::play(&input_file_path) {
                Ok(_) => (),
                Err(e) => rustii::handle_error(e, 2)
            };
        }
    };
}

