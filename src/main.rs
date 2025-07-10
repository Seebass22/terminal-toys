mod ant;
mod balls;
mod bubble;
mod cube;
mod life;
mod pipes3d;
mod sand;
mod splits;
mod tunnel;
mod utils;

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

        /// Number of balls to spawn
        #[arg(short = 'n', long, value_name = "BALLS", default_value_t = 50)]
        max_balls: u16,
    },
    /// 3d pipe screensaver
    Pipes3d {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        /// Number of segments to generate before reset
        #[arg(short = 'n', long, value_name = "SEGMENTS", default_value_t = 2000)]
        max_segments: u32,

        #[arg(short, long, value_name = "MILLISECONDS", default_value_t = 8)]
        tick_rate: u64,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u64,

        /// Use orthographic projection
        #[arg(short, long, default_value_t = false)]
        orthographic: bool,

        /// Camera speed
        #[arg(short = 'x', long, value_name = "SPEED", default_value_t = 4.0)]
        camera_speed: f64,

        /// Instead of resetting, delete earlier segments
        #[arg(short, long, default_value_t = false)]
        rotate: bool,
    },
    /// Lines that split after a while
    Splits {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        /// Lines have a random rotation
        #[arg(short, long, default_value_t = false)]
        rotate: bool,

        #[arg(short = 'n', long, value_name = "WALKERS", default_value_t = 80)]
        max_walkers: u16,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u128,
    },
    /// Game of life
    Life {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::HalfBlock)]
        marker: Marker,

        /// Width of board (default: terminal width)
        #[arg(short, long, value_name = "WIDTH")]
        width: Option<usize>,

        /// Ratio of live cells to start with
        #[arg(short, value_name = "RATIO", default_value_t = 0.5)]
        n: f32,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 3)]
        seed: u128,
    },
    /// Falling sand
    Sand {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::HalfBlock)]
        marker: Marker,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 0)]
        seed: u128,

        /// Speed multiplier
        #[arg(short = 'x', long, value_name = "MULT", default_value_t = 1)]
        speed: usize,

        /// Number of obstacles
        #[arg(short, long, value_name = "N", default_value_t = 40)]
        obstacles: usize,

        /// Length of obstacles
        #[arg(short = 'l', long, value_name = "N", default_value_t = 5)]
        obstacle_len: usize,

        /// Average number of particles to spawn before changing spawn point
        #[arg(short, long, value_name = "N", default_value_t = 100)]
        particles: u64,

        /// Flip after N ticks
        #[arg(short, long, value_name = "N")]
        flip_after: Option<u32>,

        /// reset after sand emptied N times
        #[arg(short, long, value_name = "N", default_value_t = 3)]
        reset: usize,
    },
    /// Rotating tunnel
    Tunnel {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::HalfBlock)]
        marker: Marker,

        /// Number of colors
        #[arg(short, long, value_name = "N", default_value_t = 16)]
        n_colors: u8,

        /// Rotation speed
        #[arg(short = 'x', long, value_name = "SPEED", default_value_t = 1.0)]
        speed: f64,

        /// Amount of depth (0, 1, 2)
        #[arg(short, long, default_value_t = 1)]
        depth: u8,

        /// Twisting tunnel
        #[arg(short, long, default_value_t = false)]
        twist: bool,
    },
    /// Langton's Ant
    Ant {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::HalfBlock)]
        marker: Marker,

        /// Width of board (default: terminal width)
        #[arg(short, long, value_name = "WIDTH")]
        width: Option<usize>,

        /// Speed multiplier
        #[arg(short = 'x', long, value_name = "MULT", default_value_t = 1)]
        speed: usize,

        /// Number of colors
        #[arg(short, long, value_name = "N", default_value_t = 16, value_parser = clap::value_parser!(u8).range(2..))]
        n_colors: u8,

        /// Step distance = color
        #[arg(short, long, default_value_t = false)]
        dist_by_color: bool,

        /// Fill in path
        #[arg(short, long, default_value_t = false)]
        filled: bool,

        /// Pattern of starting active blocks [default: random]
        #[arg(short, long, value_name = "INDEX", value_parser = clap::value_parser!(u8).range(0..6))]
        pattern: Option<u8>,

        /// Pattern length [default: random]
        #[arg(short = 'l', long, value_name = "N")]
        pattern_len: Option<usize>,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u128,
    },
    /// Bubble universe by A-na5 / ｱ_ﾅ
    Bubble {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::HalfBlock)]
        marker: Marker,

        /// Parameter a
        #[arg(short, value_name = "N", default_value_t = 30)]
        a: u32,

        /// Parameter b
        #[arg(short, value_name = "N", default_value_t = 30)]
        b: u32,

        /// Number of colors
        #[arg(short, long, value_name = "N", default_value_t = 16, value_parser = clap::value_parser!(u8).range(2..))]
        n_colors: u8,
    },
    /// Rotating sine wave cube
    Cube {
        /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
        #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
        marker: Marker,

        #[arg(short, long, value_name = "MILLISECONDS", default_value_t = 8)]
        tick_rate: u64,

        /// Use orthographic projection
        #[arg(short, long, default_value_t = false)]
        orthographic: bool,

        #[arg(short, long, value_name = "SPEED", default_value_t = 1.0)]
        x_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.33333)]
        y_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.0)]
        z_rotation_speed: f64,

        /// Amplitude of sine waves
        #[arg(short, long, value_name = "AMPLITUDE", default_value_t = 1.2)]
        amplitude: f64,

        /// Frequency of sine waves
        #[arg(short, long, value_name = "AMPLITUDE", default_value_t = 20.0)]
        frequency: f64,
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
            camera_speed,
            rotate,
        } => pipes3d::App::new(
            size.width,
            size.height,
            *marker,
            *max_segments,
            *orthographic,
            *rotate,
        )
        .run(terminal, *tick_rate, *seed, *camera_speed),
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
            obstacle_len,
            particles,
            flip_after,
            reset,
        } => sand::App::new(
            size.width,
            size.height,
            *marker,
            *seed,
            *speed,
            *obstacles,
            *obstacle_len,
            *particles,
            *flip_after,
            *reset,
        )
        .run(terminal),
        Commands::Tunnel {
            marker,
            n_colors,
            speed,
            depth,
            twist,
        } => tunnel::App::new(
            size.width,
            size.height,
            *marker,
            *n_colors,
            *speed,
            *depth,
            *twist,
        )
        .run(terminal),
        Commands::Ant {
            marker,
            speed,
            width,
            n_colors,
            dist_by_color,
            filled,
            pattern,
            pattern_len,
            seed,
        } => ant::App::new(
            size.width,
            size.height,
            *marker,
            *speed,
            *width,
            *n_colors,
            *dist_by_color,
            *filled,
            *pattern,
            *pattern_len,
            *seed,
        )
        .run(terminal),
        Commands::Bubble {
            marker,
            n_colors,
            a,
            b,
        } => bubble::App::new(size.width, size.height, *marker, *n_colors, *a, *b).run(terminal),
        Commands::Cube {
            marker,
            tick_rate,
            orthographic,
            x_rotation_speed,
            y_rotation_speed,
            z_rotation_speed,
            amplitude,
            frequency,
        } => cube::App::new(
            size.width,
            size.height,
            *marker,
            *orthographic,
            *x_rotation_speed,
            *y_rotation_speed,
            *z_rotation_speed,
            *amplitude,
            *frequency,
        )
        .run(terminal, *tick_rate),
    };
    ratatui::restore();
    app_result
}
