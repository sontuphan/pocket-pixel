mod config;
mod generate;
mod palette;
mod textify;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a config with the EDG32 palette and 32x32 tiles
    Init,
    /// Show the current config's colors, normals, and character mappings
    Palette,
    /// Get the current project's configuration or palette
    Get {
        #[command(subcommand)]
        resource: Resource,
    },
    /// Print instructions for agents creating pixel art with pocket-pixel
    Discover,
    /// Generate color and normal-map PNGs from a .pp.json artifact
    Generate {
        /// Artifact containing colors and normals text grids
        artifact: std::path::PathBuf,
    },
    /// Convert an image to text grids using the nearest configured palette colors
    Textify {
        /// PNG, JPEG, WebP, GIF (first frame), or BMP image
        image: std::path::PathBuf,
        /// Output .pp.json path (defaults to the input image's basename)
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
        /// Pixels with alpha at or below this value become transparent
        #[arg(long, default_value_t = 127)]
        alpha_threshold: u8,
    },
}

#[derive(Subcommand)]
enum Resource {
    /// Print the current configuration as JSON
    Config,
    /// Show colors, normals, and character mappings (alias: pocket-pixel palette)
    Palette,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Init => config::init(),
        Command::Generate { artifact } => generate::run(&artifact),
        Command::Textify {
            image,
            output,
            alpha_threshold,
        } => textify::run(&image, output.as_deref(), alpha_threshold),
        Command::Palette
        | Command::Get {
            resource: Resource::Palette,
        } => palette::show(),
        Command::Get {
            resource: Resource::Config,
        } => config::show(),
        Command::Discover => {
            println!("{}", include_str!("assets/discover.md"));
            Ok(())
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
