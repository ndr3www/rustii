use clap::Parser;

fn main() {
    let cli = rustii::Cli::parse();

    match &cli.command {
        rustii::Commands::Render { input_file_path, output_file_path, scale } => rustii::render(&input_file_path, &output_file_path, &scale),
        rustii::Commands::Play { input_file_path } => rustii::play(&input_file_path)
    };
}
