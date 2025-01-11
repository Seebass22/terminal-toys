use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::KeyEventKind;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Circle},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let size = terminal.size().unwrap();
    let app_result = App::new(size.width, size.height).run(terminal);
    ratatui::restore();
    app_result
}

struct Ball {
    circle: Circle,
    vx: f64,
    vy: f64,
}

impl Ball {
    fn new(vx: f64, vy: f64) -> Self {
        Self {
            circle: Circle {
                x: 20.0,
                y: 40.0,
                radius: 5.0,
                color: Color::Yellow,
            },
            vx,
            vy,
        }
    }
}

struct App {
    exit: bool,
    balls: Vec<Ball>,
    playground: Rect,
    tick_count: u64,
    marker: Marker,
    debug_text: String,
}

impl App {
    fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let first_ball = Ball::new(2.9, 5.0);
        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            balls: vec![first_ball],
            tick_count: 0,
            marker: Marker::Braille,
            debug_text: String::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut rng = oorandom::Rand64::new(99);
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_press(key),
                    Event::Resize(_columns, _rows) => {
                        // self.debug_text = format!("{} {}", columns, rows);
                    }
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
                if self.tick_count % 20 == 0 {
                    let x = 1.0 + 3.0 * rng.rand_float();
                    let y = 1.0 + 3.0 * rng.rand_float();
                    self.balls.push(Ball::new(x, y));
                }
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
        for ball in self.balls.iter_mut() {
            let playground = self.playground;
            if ball.circle.x - ball.circle.radius < f64::from(playground.left())
                || ball.circle.x + ball.circle.radius > f64::from(playground.right())
            {
                ball.vx = -ball.vx;
            }
            // no top barrier
            if ball.circle.y - ball.circle.radius < f64::from(playground.top()) {
                ball.vy = -ball.vy;
            }

            ball.circle.x += ball.vx;
            ball.circle.y += ball.vy;
            ball.vy -= 0.2;
            ball.vy *= 0.99;
            ball.vx *= 0.999;
            if self.tick_count % 100 == 0 {
                ball.vy *= 2.0;
                ball.vx *= 1.02;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let debug_text = Paragraph::new(self.debug_text.clone());
        frame.render_widget(self.canvas(), frame.area());
        frame.render_widget(debug_text, frame.area());
    }

    fn canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .marker(self.marker)
            .paint(|ctx| {
                for ball in self.balls.iter() {
                    ctx.draw(&ball.circle);
                }
            })
            .x_bounds([
                self.playground.left() as f64,
                self.playground.right() as f64,
            ])
            .y_bounds([
                self.playground.top() as f64,
                self.playground.bottom() as f64,
            ])
    }
}
