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
    let size = terminal.size().unwrap();
    let app_result = App::new(size.width, size.height).run(terminal);
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
    fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let height = 200.0 * scale_factor * font_scale_factor;
        Self {
            exit: false,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 5.0,
                color: Color::Yellow,
            },
            playground: Rect::new(0, 0, 200, height as u16),
            vx: 2.9,
            vy: 5.0,
            tick_count: 0,
            marker: Marker::Braille,
            debug_text: format!(
                "{} {} {} {}",
                terminal_width, terminal_height, height, height as u16
            ),
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
                        self.debug_text = format!("{} {}", columns, rows);
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
            KeyCode::Char('Q') => self.exit = true,
            _ => (),
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
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
        self.vy -= 0.2;
        self.vy *= 0.99;
        self.vx *= 0.999;
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
