mod balls;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result = balls::App::new(size.width, size.height).run(terminal);
    ratatui::restore();
    app_result
}
