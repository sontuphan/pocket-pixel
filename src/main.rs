mod config;
mod palette;

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
