mod balls;
mod life;
mod pipes3d;
mod sand;
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

        /// Width of board
        #[arg(short, long, value_name = "WIDTH", default_value_t = 40)]
        width: usize,

        /// Number of live cells to start with
        #[arg(short, value_name = "N", default_value_t = 300)]
        n: usize,

        #[arg(short, long, value_name = "SEED", default_value_t = 3)]
        seed: u128,
    },
    /// Falling sand
    Sand {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        #[arg(short, long, value_name = "SEED", default_value_t = 0)]
        seed: u128,

        #[arg(short = 'x', long, value_name = "MULT", default_value_t = 2)]
        speed: usize,

        #[arg(short, long, value_name = "N", default_value_t = 40)]
        obstacles: usize,

        /// Average number of particles to spawn before changing spawn point
        #[arg(short, long, value_name = "N", default_value_t = 100)]
        particles: u64,
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
        Commands::Life {
            marker,
            seed,
            n,
            width,
        } => life::App::new(size.width, size.height, *marker, *seed, *n, *width).run(terminal),
        Commands::Sand {
            marker,
            seed,
            speed,
            obstacles,
            particles,
        } => sand::App::new(
            size.width,
            size.height,
            *marker,
            *seed,
            *speed,
            *obstacles,
            *particles,
        )
        .run(terminal),
    };
    ratatui::restore();
    app_result
}
