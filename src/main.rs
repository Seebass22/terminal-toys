mod ant;
mod balls;
mod bubble;
mod cube;
mod life;
mod pipes3d;
mod rings;
mod sand;
mod sparks;
mod splits;
mod tunnel;
mod utils;

use color_eyre::Result;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell};
use ratatui::symbols::Marker;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Marker type (Braille, Dot, Bar, Block, HalfBlock, Quadrant, Sextant, Octant)
    #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille, global = true)]
    marker: Marker,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Bouncy balls!
    Balls {
        /// Number of balls to spawn
        #[arg(short = 'n', long, value_name = "BALLS", default_value_t = 50)]
        max_balls: u16,
    },
    /// 3d pipe screensaver
    Pipes3d {
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

        /// Reset after sand emptied N times
        #[arg(short, long, value_name = "N", default_value_t = 3)]
        reset: usize,

        /// Obstacle color index
        #[arg(short = 'c', long, value_name = "INDEX", default_value_t = 7)]
        obstacle_color: u8,
    },
    /// Rotating tunnel
    Tunnel {
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
        #[arg(short, long, value_name = "MILLISECONDS", default_value_t = 8)]
        tick_rate: u64,

        /// Use orthographic projection
        #[arg(short, long, default_value_t = false)]
        orthographic: bool,

        #[arg(short, long, value_name = "SPEED", default_value_t = 1.0)]
        x_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.5)]
        y_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.25)]
        z_rotation_speed: f64,

        /// Amplitude of sine waves
        #[arg(short, long, value_name = "AMPLITUDE", default_value_t = 1.2)]
        amplitude: f64,

        /// Frequency of sine waves
        #[arg(short, long, value_name = "FREQUENCY", default_value_t = 1.0)]
        frequency: f64,

        /// Speed of sine wave phase shift
        #[arg(short, long, value_name = "SPEED", default_value_t = 1.0)]
        speed: f64,

        /// Color change speed
        #[arg(short, long, value_name = "SPEED")]
        color_speed: Option<f64>,
    },
    /// Sphere made out of shifting rings
    Rings {
        #[arg(short, long, value_name = "MILLISECONDS", default_value_t = 8)]
        tick_rate: u64,

        /// Use orthographic projection
        #[arg(short, long, default_value_t = false)]
        orthographic: bool,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.5)]
        x_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.25)]
        y_rotation_speed: f64,

        #[arg(short, long, value_name = "SPEED", default_value_t = 0.125)]
        z_rotation_speed: f64,

        /// Amplitude of sine waves
        #[arg(short, long, value_name = "AMPLITUDE", default_value_t = 5.0)]
        amplitude: f64,

        /// Frequency of sine waves
        #[arg(short, long, value_name = "FREQUENCY", default_value_t = 10.0)]
        frequency: f64,

        /// Speed of sine wave phase shift
        #[arg(short, long, value_name = "SPEED", default_value_t = 0.1)]
        speed: f64,

        /// Zoom
        #[arg(
            long,
            value_name = "ZOOM",
            default_value_t = 0.0,
            allow_negative_numbers = true
        )]
        zoom: f64,
    },
    /// Explosions!
    Sparks {
        /// Number of sparks per explosion
        #[arg(short = 'n', long, value_name = "SPARKS", default_value_t = 360)]
        n_sparks: usize,

        /// RNG seed
        #[arg(short, long, value_name = "SEED", default_value_t = 99)]
        seed: u128,

        /// Spark lifetime in ticks
        #[arg(short, long, value_name = "TICKS", default_value_t = 100)]
        lifetime: u64,

        /// Number of ticks to wait between spawning explosions
        #[arg(short, long, value_name = "TICKS", default_value_t = 100)]
        rate: u64,

        /// Explosion power
        #[arg(short, long, value_name = "POWER", default_value_t = 10.0)]
        power: f64,

        /// Random offset added to particle velocity
        #[arg(short, long, value_name = "POWER", default_value_t = 3.0)]
        velocity_offset: f64,
    },
    Completion {
        shell: Shell,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    if let Commands::Completion { shell } = cli.command {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return Ok(());
    }
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result = match &cli.command {
        Commands::Balls { max_balls } => {
            balls::App::new(size.width, size.height, cli.marker, *max_balls).run(terminal)
        }
        Commands::Pipes3d {
            max_segments,
            tick_rate,
            seed,
            orthographic,
            camera_speed,
            rotate,
        } => pipes3d::App::new(
            size.width,
            size.height,
            cli.marker,
            *max_segments,
            *orthographic,
            *rotate,
        )
        .run(terminal, *tick_rate, *seed, *camera_speed),
        Commands::Splits {
            rotate,
            max_walkers,
            seed,
        } => splits::App::new(
            size.width,
            size.height,
            cli.marker,
            *rotate,
            *max_walkers,
            *seed,
        )
        .run(terminal),
        Commands::Life { seed, n, width } => {
            life::App::new(size.width, size.height, cli.marker, *seed, *n, *width).run(terminal)
        }
        Commands::Sand {
            seed,
            speed,
            obstacles,
            obstacle_len,
            particles,
            flip_after,
            reset,
            obstacle_color,
        } => sand::App::new(
            size.width,
            size.height,
            cli.marker,
            *seed,
            *speed,
            *obstacles,
            *obstacle_len,
            *particles,
            *flip_after,
            *reset,
            *obstacle_color,
        )
        .run(terminal),
        Commands::Tunnel {
            n_colors,
            speed,
            depth,
            twist,
        } => tunnel::App::new(
            size.width,
            size.height,
            cli.marker,
            *n_colors,
            *speed,
            *depth,
            *twist,
        )
        .run(terminal),
        Commands::Ant {
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
            cli.marker,
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
        Commands::Bubble { n_colors, a, b } => {
            bubble::App::new(size.width, size.height, cli.marker, *n_colors, *a, *b).run(terminal)
        }
        Commands::Cube {
            tick_rate,
            orthographic,
            x_rotation_speed,
            y_rotation_speed,
            z_rotation_speed,
            amplitude,
            frequency,
            speed,
            color_speed,
        } => cube::App::new(
            size.width,
            size.height,
            cli.marker,
            *orthographic,
            *x_rotation_speed,
            *y_rotation_speed,
            *z_rotation_speed,
            *amplitude,
            *frequency,
            *speed,
            *color_speed,
        )
        .run(terminal, *tick_rate),
        Commands::Rings {
            tick_rate,
            orthographic,
            x_rotation_speed,
            y_rotation_speed,
            z_rotation_speed,
            amplitude,
            frequency,
            speed,
            zoom,
        } => rings::App::new(
            size.width,
            size.height,
            cli.marker,
            *orthographic,
            *x_rotation_speed,
            *y_rotation_speed,
            *z_rotation_speed,
            *amplitude,
            *frequency,
            *speed,
            *zoom,
        )
        .run(terminal, *tick_rate),
        Commands::Sparks {
            n_sparks,
            seed,
            lifetime,
            rate,
            power,
            velocity_offset,
        } => sparks::App::new(
            size.width,
            size.height,
            cli.marker,
            *n_sparks,
            *seed,
            *lifetime,
            *rate,
            *power,
            *velocity_offset,
        )
        .run(terminal),
        Commands::Completion { shell: _shell } => unreachable!(),
    };
    ratatui::restore();
    app_result
}

fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    // workaround for https://github.com/clap-rs/clap/issues/6421
    let cmd_name = cmd.get_name().to_owned();
    let cmd_name_without_hyphen = cmd_name.replace("-", "_");
    clap_complete::generate(
        generator,
        cmd,
        cmd_name_without_hyphen.clone(),
        &mut std::io::stdout(),
    );
    println!("complete -F _{cmd_name_without_hyphen} {cmd_name}");
}
