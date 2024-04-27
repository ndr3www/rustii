use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::str;

use clap::{Parser, Subcommand};
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use indicatif::{ProgressBar, ProgressStyle};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

const GRAYSCALE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

const SPINNER_TICK: u64 = 80;

/// Handles parsing of command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

impl Cli {
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
}

/// List of available commands and options
#[derive(Subcommand)]
pub enum Commands {
    /// Render specified image file to ASCII art
    Render {
        /// Image file to be rendered
        input_file_path: String,
        /// Produced ASCII art file
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output: String,
        /// Scale of the produced ASCII art in order: <WIDTH_SCALE> <HEIGHT_SCALE>
        #[arg(short, long, value_name = "FLOAT", value_parser, num_args = 2, value_delimiter = ' ')]
        #[arg(default_values_t = [1.0, 1.0])]
        scale: Vec<f32>,
        /// Adjust the contrast of the produced ASCII art, negative values decrease the contrast and positive increase it
        #[arg(short, long, value_name = "FLOAT")]
        #[arg(value_parser = clap::value_parser!(f32))]
        #[arg(default_value_t = 0.0)]
        #[arg(allow_hyphen_values = true)]
        contrast: f32
    },
    /// Open specified ASCII art file in terminal
    Open {
        input_file_path: String
    }
}

/// Handles conversion of a given image file to ASCII art file
pub fn render(input_file_path: &String, output_file_path: &String, scale: &[f32], contrast: &f32) -> Result<(), String> {
    // Scale validation
    if scale[0] < 0.0 || scale[1] < 0.0 {
        return Err(String::from("Scale cannot be negative"));
    }

    // Read raw image data from the file
    let img = ImageReader::open(input_file_path).map_err(|e| format!("{input_file_path}: {e}"))?;

    // Set up and enable progress indicator
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner:.default} {msg}").unwrap().tick_strings(&[
                        "[    ]",
                        "[=   ]",
                        "[==  ]",
                        "[=== ]",
                        "[====]",
                        "[ ===]",
                        "[  ==]",
                        "[   =]",
                        "[    ]",
                        "[   =]",
                        "[  ==]",
                        "[ ===]",
                        "[====]",
                        "[=== ]",
                        "[==  ]",
                        "[=   ]",
                        "[====]"
                        ])
                      );
    spinner.set_message("Decoding");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_TICK));

    // Decode the raw image
    let mut img_decoded = img.decode().map_err(|e| format!("{input_file_path}: {e}"))?;

    spinner.set_message("Processing");

    // Image processing
    img_decoded = img_decoded
        .resize_exact(
            (img_decoded.width() as f32 * scale[0]) as u32,
            (img_decoded.height() as f32 * scale[1]) as u32,
            FilterType::Nearest
        )
        .grayscale()
        .filter3x3(&[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0])
        .adjust_contrast(contrast.to_owned());

    spinner.set_message("Conversion");

    // Conversion to ASCII art
    let mut ascii_img = convert_to_ascii(img_decoded);

    // Add metadata
    ascii_img.append(&mut format!("Scale: {}, {}\nContrast: {contrast}", scale[0], scale[1]).as_bytes().to_vec());

    spinner.set_message("Compression");

    // Compression
    ascii_img = compress_to_vec(&ascii_img, 10);

    // Disable the progress indicator
    spinner.finish_with_message("Done");

    // Create/open the output file for writing
    let mut output_file = File::create(output_file_path).map_err(|e| format!("{output_file_path}: {e}"))?;

    // Write data to the output file
    output_file.write_all(&ascii_img).map_err(|e| format!("{output_file_path}: {e}"))?;

    Ok(())
}

fn convert_to_ascii(image: DynamicImage) -> Vec<u8> {
    let mut ascii_image = Vec::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            // Map ASCII grayscale characters to pixel values
            ascii_image.push(GRAYSCALE
                             .as_bytes()
                             [
                                usize::from(image.get_pixel(x, y).channels()[0]) / 4
                             ]
            );
        }

        // Add newline at the end
        ascii_image.push(b'\n');
    }

    ascii_image
}

/// Reads the contents of a given ASCII art file and prints it to the standard output
pub fn open(input_file_path: &String) -> Result<(), String> {
    // Open the file containing compressed ASCII art
    let mut input_file = File::open(input_file_path).map_err(|e| format!("{input_file_path}: {e}"))?;

    let mut contents = Vec::new();

    // Read the contents of the file
    input_file.read_to_end(&mut contents).map_err(|e| format!("{input_file_path}: {e}"))?;

    // Decompress read data
    contents = decompress_to_vec(&contents).map_err(|e| format!("{e}"))?;

    // Decode the data as string
    let contents_str = str::from_utf8(&contents).map_err(|e| format!("{e}"))?;

    // Print the string to the standard output
    println!("{contents_str}");

    Ok(())
}

/// Prints given error message to the standard error with application name and then exits the application with specified error code
pub fn handle_error(error: String, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}

