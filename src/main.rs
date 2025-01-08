//! # [Ratatui] Canvas example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::{
    io::stdout,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},
    ExecutableCommand,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Circle},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    stdout().execute(DisableMouseCapture)?;
    app_result
}

struct App {
    exit: bool,
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    marker: Marker,
    debug_text: String,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 2.0,
                color: Color::Yellow,
            },
            playground: Rect::new(0, 0, 200, 100),
            vx: 3.0,
            vy: 3.0,
            tick_count: 0,
            marker: Marker::Dot,
            debug_text: String::from("test"),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_press(key),
                    Event::Resize(columns, rows) => {
                        // dbg!(columns, rows);
                        self.debug_text = format!("{} {}", columns, rows);
                        self.playground.width = columns - 2;
                        self.playground.height = rows - 2;
                        // self.ball.x.clamp(ball.radius, max)
                    }
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            _ => (),
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        // only change marker every 180 ticks (3s) to avoid stroboscopic effect
        if (self.tick_count % 180) == 0 {
            self.marker = match self.marker {
                Marker::Dot => Marker::Braille,
                Marker::Braille => Marker::Block,
                Marker::Block => Marker::HalfBlock,
                Marker::HalfBlock => Marker::Bar,
                Marker::Bar => Marker::Dot,
            };
        }
        // bounce the ball by flipping the velocity vector
        let ball = &self.ball;
        let playground = self.playground;
        if ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
        {
            self.vx = -self.vx;
        }
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }

        self.ball.x += self.vx;
        self.ball.y += self.vy;
    }

    fn ball_out_of_bounds(ball: Circle, playground: Rect) -> bool {
        return ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
            || ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom());
    }

    fn draw(&self, frame: &mut Frame) {
        let greeting = Paragraph::new(self.debug_text.clone());
        frame.render_widget(self.pong_canvas(), frame.area());
        frame.render_widget(greeting, frame.area());
    }

    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Pong"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball);
            })
            .x_bounds([0.0, self.playground.width as f64])
            .y_bounds([0.0, self.playground.height as f64])
    }
}
