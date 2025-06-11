use std::{
    io::ErrorKind,
    path::{self, PathBuf},
};

use ::clap::Parser;

mod clap;
mod colors;
use ansi_term::Colour::{Black, Cyan, Green, RGB, Red, White};
use clap::{Cli, Commands};
use colors::colors::get_palette;
use image::{ImageError, ImageReader, Rgb, RgbImage};

fn main() {
    let cli = Cli::parse();

    let quantity = cli.quantity.unwrap_or(5);
    let tolerance = cli.tolerance.unwrap_or(8.0);

    let result = match &cli.command {
        Commands::Extract { path } => extract::<ImageError>(path, tolerance, quantity),
        Commands::Image { input, output } => image(input, output, tolerance, quantity),
        Commands::Json { path } => json(path, tolerance, quantity),
    };

    if let Err(error) = result {
        println!(
            "{}",
            Red.paint(format!("An error has ocurred! {}", error.to_string()))
        );
    }
}

fn extract<T: From<std::io::Error> + From<ImageError>>(
    path: &PathBuf,
    tolerance: f32,
    quantity: u16,
) -> Result<(), T> {
    let image = ImageReader::open(path)?.decode()?;
    let palette = get_palette(image, tolerance);

    println!(
        "{} different colors have been extracted!\nShowing the {} most prevalent:\n",
        Cyan.underline().paint(format!("{}", palette.len())),
        Green.underline().paint(format!("{}", quantity)),
    );

    let mut quantity = quantity as usize;

    if palette.len() < quantity as usize {
        println!(
            "{}",
            Red.paint(format!(
                "Not enough colors could be extracted, capping to {}",
                palette.len()
            ))
        );
        quantity = palette.len()
    }

    for color in 0..quantity {
        let color = palette[color];

        let foreground = if color.get_rel_lum() < 0.5 {
            White
        } else {
            Black
        };
        let text = foreground.on(RGB(color.0, color.1, color.2)).paint(format!(
            "#{:X}{:X}{:X}\t{}, {}, {}",
            color.0, color.1, color.2, color.0, color.1, color.2
        ));
        println!("{text}")
    }
    Ok(())
}

fn image<T: From<std::io::Error> + From<ImageError>>(
    input: &PathBuf,
    output: &Option<PathBuf>,
    tolerance: f32,
    quantity: u16,
) -> Result<(), T> {
    let image = ImageReader::open(input)?.decode()?;
    let palette = get_palette(image, tolerance);

    let width = 32 * quantity as u32;
    let mut img = RgbImage::new(width, 32);
    let mut color_index: usize = 0;

    let mut quantity = quantity as usize;

    if palette.len() < quantity as usize {
        println!(
            "{}",
            Red.paint(format!(
                "Not enough colors could be extracted, capping to {}",
                palette.len()
            ))
        );
        quantity = palette.len()
    }

    for x in 0..width {
        if color_index == quantity && x == width - 1 {
            color_index = 0;
        } else if x % 32 == 0 && x != 0 {
            color_index += 1;
        }
        let color = palette[color_index];
        let (r, g, b) = (color.0, color.1, color.2);
        for y in 0..32 {
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    let path: PathBuf = [r"./", "new_palette.png"].iter().collect();
    let output = output.clone().unwrap_or(path);

    img.save(&output)?;

    println!(
        "{}",
        Green.paint(format!(
            "Image was saved to {}",
            path::absolute(&output).unwrap_or(output).to_str().unwrap()
        ))
    );
    Ok(())
}

fn json<T: From<std::io::Error> + From<ImageError>>(
    path: &PathBuf,
    tolerance: f32,
    quantity: u16,
) -> Result<(), T> {
    let image = ImageReader::open(path)?.decode()?;
    let mut palette = get_palette(image, tolerance);
    palette.truncate(quantity as usize);

    let Ok(json) = serde_json::to_string(&palette) else {
        return Err(std::io::Error::new(ErrorKind::Other, "Json could'nt be created").into());
    };
    println!("{}", json);
    Ok(())
}
