use clap::Parser;

pub const APP_NAME: &str = "rustii";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    render: String,

    #[arg(long)]
    output: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_render() {
        Cli::parse_from(vec![APP_NAME, "--render", "image.png", "--output", "image.txt"]);
    }
}
