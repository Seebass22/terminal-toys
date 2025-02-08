mod balls;
mod life;
mod pipes3d;
mod splits;

use color_eyre::Result;

use clap::{Parser, Subcommand};
use ratatui::symbols::Marker;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

        #[arg(short = 'n', long, value_name = "SEGMENTS", default_value_t = 20000)]
        max_segments: u32,

        #[arg(short, long, value_name = "MILLISECONDS", default_value_t = 8)]
        tick_rate: u64,

        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u64,

        #[arg(short, long, default_value_t = false)]
        orthographic: bool,
    },
    /// Lines that split after a while
    Splits {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        #[arg(short, long, default_value_t = false)]
        rotate: bool,

        #[arg(short = 'n', long, value_name = "WALKERS", default_value_t = 80)]
        max_walkers: u16,

        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u128,
    },
    /// Game of life
    Life {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u128,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result = match &cli.command {
        Commands::Balls { marker, max_balls } => {
            balls::App::new(size.width, size.height, *marker, *max_balls).run(terminal)
        }
        Commands::Pipes3d {
            marker,
            max_segments,
            tick_rate,
            seed,
            orthographic,
        } => pipes3d::App::new(
            size.width,
            size.height,
            *marker,
            *max_segments,
            *orthographic,
        )
        .run(terminal, *tick_rate, *seed),
        Commands::Splits {
            marker,
            rotate,
            max_walkers,
            seed,
        } => splits::App::new(
            size.width,
            size.height,
            *marker,
            *rotate,
            *max_walkers,
            *seed,
        )
        .run(terminal),
        Commands::Life { marker, seed } => {
            life::App::new(size.width, size.height, *marker, *seed).run(terminal)
        }
    };
    ratatui::restore();
    app_result
}
