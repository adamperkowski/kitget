use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

const LICENSE: &str = include_str!("../LICENSE");

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// animated kitteh WOW :3
    #[arg(short, long)]
    pub gif: bool,

    /// Comma-separated list of tags
    #[arg(short, long)]
    pub tags: Option<String>,

    /// Text for the cat to say
    #[arg(short, long)]
    pub display: Option<String>,

    /// Only square images
    #[arg(short, long)]
    pub square: bool,

    /// Monochrome filter
    #[arg(short, long)]
    pub mono: bool,

    /// Width (in pixels)
    #[arg(long)]
    pub width: Option<usize>,

    /// Height (in pixels)
    #[arg(long)]
    pub height: Option<usize>,

    /// Amount of blur (0.3-1000)
    #[arg(long)]
    pub blur: Option<f32>,

    /// Amount of redness (0-255)
    #[arg(long)]
    pub red: Option<u8>,

    /// Amount of greenness (0-255)
    #[arg(long)]
    pub green: Option<u8>,

    /// Amount of blueness (0-255)
    #[arg(long)]
    pub blue: Option<u8>,

    /// Amount of brightness
    #[arg(long)]
    pub brightness: Option<f32>,

    /// Amount of saturation
    #[arg(long)]
    pub saturation: Option<f32>,

    /// Amount of hue
    #[arg(long)]
    pub hue: Option<f32>,

    /// Amount of lightness
    #[arg(long)]
    pub lightness: Option<f32>,

    /// Filepath to save the image to
    #[arg(short, long)]
    pub output: Option<String>,

    /// Print position row
    #[arg(short, long)]
    pub row: Option<i16>,

    /// Print position column
    #[arg(short, long)]
    pub column: Option<u16>,

    /// Print size X
    #[arg(short)]
    pub x: Option<u32>,

    /// Print size Y
    #[arg(short)]
    pub y: Option<u32>,

    /// Display copyright information
    #[arg(long)]
    copyright: bool,
}

impl Args {
    pub fn get() -> Self {
        let args = Args::parse();

        if args.copyright {
            let current_year = 1970
                + (SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards :(")
                    .as_secs()
                    / (365 * 24 * 60 * 60));

            let copyright = if current_year > 2025 {
                &format!("Copyright (c) 2025-{current_year} Adam Perkowski")
            } else {
                "Copyright (c) 2025 Adam Perkowski"
            };

            println!(
                "{copyright}\n\n{}",
                LICENSE.lines().skip(4).collect::<Vec<_>>().join("\n")
            );

            std::process::exit(0);
        }

        args
    }
}
