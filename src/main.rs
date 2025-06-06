use std::{fmt::format, path::PathBuf};

use ::clap::Parser;

mod clap;
mod colors;
use ansi_term::Colour::{Black, Cyan, Green, RGB, White};
use clap::{Cli, Commands};
use colors::colors::get_palette;
use image::{ImageError, ImageReader};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Extract { path } => extract::<ImageError>(path),
    };
}

fn extract<T: From<std::io::Error> + From<ImageError>>(path: &PathBuf) -> Result<(), T> {
    let image = ImageReader::open(path)?.decode()?;
    let palette = get_palette(image);

    println!(
        "{} different colors have been extracted!\nShowing the {} most prevalent!\n",
        Cyan.underline().paint(format!("{}", palette.len())),
        Green.underline().paint(format!("{}", 5)),
    );

    for color in 0..5 {
        let color = palette[color];

        let foreground = if color.get_lum() < 0.5 { White } else { Black };
        let text = foreground.on(RGB(color.0, color.1, color.2)).paint(format!(
            "#{:X}{:X}{:X}\t{}, {}, {}",
            color.0, color.1, color.2, color.0, color.1, color.2
        ));
        println!("{text}")
    }
    Ok(())
}
