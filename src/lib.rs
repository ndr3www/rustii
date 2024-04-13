use clap::{Parser, Subcommand};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Render specified media file to ASCII art
    Render {
        /// Image or video to be rendered
        input_file_path: Option<String>,
        /// ASCII art file path
        output_file_path: Option<String>,
        /// Scale of the final ASCII art, default is 1.0
        scale: Option<f32>
    },
    /// Play specified ASCII art in terminal
    Play {
        /// ASCII image or video to be played
        input_file_path: Option<String>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_render() {
        let cli = Cli::parse_from([APP_NAME, "render", "image.png", "image.txt", "1.0"]);
        
        match &cli.command {
            Commands::Render { input_file_path, output_file_path, scale } => {
                assert_eq!(input_file_path, &Some("image.png".to_string()));
                assert_eq!(output_file_path, &Some("image.txt".to_string()));
                assert_eq!(scale, &Some(1.0));
            },
            _ => ()
        };
    }

    #[test]
    fn case_play() {
        let cli = Cli::parse_from([APP_NAME, "play", "image.txt"]);
        
        match &cli.command {
            Commands::Play { input_file_path } => {
                assert_eq!(input_file_path, &Some("image.txt".to_string()));
            },
            _ => ()
        };
    }
}
