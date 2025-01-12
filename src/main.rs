mod balls;
mod pipes3d;

use color_eyre::Result;

use clap::{Parser, Subcommand};
use ratatui::symbols::Marker;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Bouncy balls!
    Balls {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        #[arg(short = 'n', long, value_name = "BALLS", default_value_t = 50)]
        max_balls: u16,
    },
    /// 3d pipe screensaver
    Pipes3d {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result = match &cli.command {
        Some(Commands::Balls { marker, max_balls }) => {
            balls::App::new(size.width, size.height, *marker, *max_balls).run(terminal)
        }
        Some(Commands::Pipes3d { marker }) => {
            pipes3d::App::new(size.width, size.height, *marker).run(terminal)
        }
        _ => Ok(()),
    };
    ratatui::restore();
    app_result
}
