mod balls;

use color_eyre::Result;

use clap::Parser;
use ratatui::symbols::Marker;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Marker type (Braille, Dot, Bar, Block, HalfBlock)
    #[arg(short, long, value_name = "TYPE", default_value_t = Marker::Braille)]
    marker: Marker,

    #[arg(short = 'n', long, value_name = "BALLS", default_value_t = 50)]
    max_balls: u16,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result =
        balls::App::new(size.width, size.height, cli.marker, cli.max_balls).run(terminal);
    ratatui::restore();
    app_result
}
