use std::process;

use clap::{Parser, Subcommand};
use image::{io::Reader as ImageReader, GenericImageView, Pixel};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

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

pub fn render(input_file_path: &String, output_file_path: &String, scale: &f32) -> Result<(), &'static str> {
    let img = match ImageReader::open(input_file_path) {
        Ok(i) => i,
        Err(e) => {
            let s: &'static str = format!("{e}: {input_file_path}").leak();
            return Err(s);
        }
    };

    let img_decoded = match img.decode() {
        Ok(i) => i,
        Err(e) => {
            let s: &'static str = format!("{e}").leak();
            return Err(s);
        }
    };

    for x in 0..img_decoded.width() {
        for y in 0..img_decoded.height() {
            for i in img_decoded.get_pixel(x, y).channels().iter() {
                println!("{i}");
            }
        }
    }

    Ok(())
}

pub fn play(input_file_path: &String) -> Result<(), &'static str> {
    Ok(())
}

pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_render() {
        let cli = Cli::parse_from([APP_NAME, "render", "image.jpeg", "image.txt"]);
        
        match &cli.command {
            Commands::Render { input_file_path, output_file_path, scale } => {
                assert_eq!(render(input_file_path, output_file_path, scale), Ok(()));
            },
            _ => ()
        };
    }

    #[test]
    fn case_play() {
        let cli = Cli::parse_from([APP_NAME, "play", "image.txt"]);
        
        match &cli.command {
            Commands::Play { input_file_path } => {
                assert_eq!(play(input_file_path), Ok(()));
            },
            _ => ()
        };
    }
}
