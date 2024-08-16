use clap::Parser;

use rustii::*;

#[test]
fn case_render() {
    let cli = Cli::parse_from([APP_NAME, "render", "tests/image.jpg", "-o", "tests/ascii.txt", "-s", "0.65", "0.25", "-c", "20"]);
    
    match cli.get_command() {
        Commands::Render { input_file_path, output, scale, contrast } => {
            assert_eq!(render(input_file_path, output, scale, contrast), Ok(()));
        },
        _ => ()
    };
}

#[test]
fn case_open() {
    let cli = Cli::parse_from([APP_NAME, "open", "tests/ascii.txt"]);
    
    match cli.get_command() {
        Commands::Open { input_file_path } => {
            assert_eq!(open(input_file_path), Ok(()));
        },
        _ => ()
    };
}

