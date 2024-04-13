use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use clap::{Parser, Subcommand};
use image::{imageops::FilterType::Triangle, io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

const GRAYSCALE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

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
        /// Scale of the width of the final ASCII art as a floating point number
        #[arg(value_parser = clap::value_parser!(f32))]
        #[arg(default_value_t = 1.0)]
        scale_width: f32,
        /// Scale of the width of the final ASCII art as a floating point number
        #[arg(value_parser = clap::value_parser!(f32))]
        #[arg(default_value_t = 1.0)]
        scale_height: f32
    },
    /// Play specified ASCII art in terminal
    Play {
        /// ASCII image or video to be played
        input_file_path: String
    }
}

pub fn render(input_file_path: &String, output_file_path: &String, scale_width: &f32, scale_height: &f32) -> Result<(), &'static str> {
    let img = match ImageReader::open(input_file_path) {
        Ok(i) => i,
        Err(e) => {
            let s: &'static str = format!("{input_file_path}: {e}").leak();
            return Err(s);
        }
    };

    let mut img_decoded = match img.decode() {
        Ok(i) => i,
        Err(e) => {
            let s: &'static str = format!("{e}").leak();
            return Err(s);
        }
    };

    img_decoded = img_decoded
        .grayscale()
        .resize_exact(
            (img_decoded.width() as f32 * scale_width) as u32,
            (img_decoded.height() as f32 * scale_height) as u32,
            Triangle
        );

    let ascii_img = convert_to_ascii(img_decoded);

    let mut output_file = match File::create(output_file_path) {
        Ok(f) => f,
        Err(e) => {
            let s: &'static str = format!("{output_file_path}: {e}").leak();
            return Err(s);
        }
    };

    match output_file.write_all(&ascii_img) {
        Ok(_) => (),
        Err(e) => {
            let s: &'static str = format!("{output_file_path}: {e}").leak();
            return Err(s);
        }
    };

    Ok(())
}

fn convert_to_ascii(image: DynamicImage) -> Vec<u8> {
    let mut ascii_image = Vec::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            ascii_image.push(GRAYSCALE
                             .as_bytes()
                             [
                                usize::try_from(image.get_pixel(x, y).channels()[0]).
                                unwrap() / 4
                             ]
            );
        }

        ascii_image.push('\n' as u8);
    }

    ascii_image
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
        let cli = Cli::parse_from([APP_NAME, "render", "image_dark.png", "image.txt"]);
        
        match &cli.command {
            Commands::Render { input_file_path, output_file_path, scale_width, scale_height } => {
                assert_eq!(render(input_file_path, output_file_path, scale_width, scale_height), Ok(()));
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
