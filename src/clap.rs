use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Number of colors to show on the list")]
    pub quantity: Option<u16>,
    #[arg(short, long, help = "Higher values will extract less colors")]
    pub tolerance: Option<f32>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Extract {
        #[arg(index = 1, help = "Path of an image")]
        path: PathBuf,
    },
    Json {
        #[arg(index = 1, help = "Path of an image")]
        path: PathBuf,
    },
    Image {
        #[arg(index = 1, help = "Path of input image")]
        input: PathBuf,
        #[arg(short, long, help = "Path of output image")]
        output: Option<PathBuf>,
    },
}
