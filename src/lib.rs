use clap::{Parser, Subcommand};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Render specified media file to ASCII art
    Render {
        /// Image or video to be rendered
        input_file_path: String,
        /// ASCII art file path
        output_file_path: String,
        /// Scale of the final ASCII art as a floating point number
        #[arg(value_parser = clap::value_parser!(f32))]
        #[arg(default_value_t = 1.0)]
        scale: f32
    },
    /// Play specified ASCII art in terminal
    Play {
        /// ASCII image or video to be played
        input_file_path: String
    }
}

pub fn render(input_file_path: &String, output_file_path: &String, scale: &f32) {

}

pub fn play(input_file_path: &String) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_render() {
        let cli = Cli::parse_from([APP_NAME, "render", "image.png", "image.txt"]);
        
        match &cli.command {
            Commands::Render { input_file_path, output_file_path, scale } => {
                assert_eq!(input_file_path, &"image.png".to_string());
                assert_eq!(output_file_path, &"image.txt".to_string());
                assert_eq!(scale, &1.0);
            },
            _ => ()
        };
    }

    #[test]
    fn case_play() {
        let cli = Cli::parse_from([APP_NAME, "play", "image.txt"]);
        
        match &cli.command {
            Commands::Play { input_file_path } => {
                assert_eq!(input_file_path, &"image.txt".to_string());
            },
            _ => ()
        };
    }
}
