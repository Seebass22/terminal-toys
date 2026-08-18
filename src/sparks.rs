use crate::utils::is_quit_key;
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use oorandom::Rand64;
use ratatui::{
    crossterm::event::{self, Event},
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    lifetime: u64,
    color: u8,
}

impl Ball {
    fn new(x: f64, y: f64, vx: f64, vy: f64, lifetime: u64, color: u8) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            lifetime,
            color,
        }
    }
}

pub struct App {
    exit: bool,
    balls: Vec<Ball>,
    playground: Rect,
    tick_count: u64,
    debug_text: String,
    marker: Marker,
    n_sparks: usize,
    rng: Rand64,
    lifetime: u64,
    rate: u64,
    power: f64,
    velocity_offset: f64,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        n_sparks: usize,
        seed: u128,
        lifetime: u64,
        rate: u64,
        power: f64,
        velocity_offset: f64,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let rng = oorandom::Rand64::new(seed);
        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            balls: Vec::new(),
            tick_count: 0,
            marker,
            debug_text: String::new(),
            n_sparks,
            rng,
            lifetime,
            rate,
            power,
            velocity_offset,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let lifetime_lower_bound = self.lifetime.saturating_sub(50);
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
                self.balls.retain(|b| b.lifetime > 0);
                for ball in self.balls.iter_mut() {
                    ball.lifetime -= 1;
                }
                if self.tick_count.is_multiple_of(self.rate) || self.tick_count == 1 {
                    let x = self.rng.rand_range(
                        (self.playground.left() as u64)..(self.playground.right() as u64),
                    ) as f64;
                    let y = self.rng.rand_range(
                        (self.playground.top() as u64)..(self.playground.bottom() as u64),
                    ) as f64;

                    let color = self.rng.rand_range(1..16) as u8;
                    let power_fac = self.rng.rand_float() * 0.5;
                    let vx_add =
                        self.rng.rand_float() * self.velocity_offset - (0.5 * self.velocity_offset);
                    let vy_add = self.rng.rand_float() * 0.5 * self.velocity_offset;

                    for phi in 0..self.n_sparks {
                        let fac = 360.0 / self.n_sparks as f64;
                        let lifetime = self.rng.rand_range(lifetime_lower_bound..self.lifetime);
                        let power = self.rng.rand_float() * self.power * power_fac;
                        let vx = vx_add + power * (fac * phi as f64).to_radians().cos();
                        let vy = vy_add + power * (fac * phi as f64).to_radians().sin();
                        self.balls.push(Ball::new(x, y, vx, vy, lifetime, color));
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if is_quit_key(key) {
            self.exit = true;
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        for ball in self.balls.iter_mut() {
            let playground = self.playground;
            if ball.x < f64::from(playground.left()) || ball.x > f64::from(playground.right()) {
                let velocity_loss_offset = self.rng.rand_float() * 0.1;
                ball.vx = -(0.9 + velocity_loss_offset) * ball.vx;
            }
            // no top barrier
            if ball.y < f64::from(playground.top()) {
                let velocity_loss_offset = self.rng.rand_float() * 0.1;
                ball.vy = (0.75 + velocity_loss_offset) * -ball.vy;
            }

            ball.x += ball.vx;
            ball.y += ball.vy;
            ball.vy -= 0.2;
            if ball.vy > 0.0 {
                ball.vy *= 0.98;
            }
            ball.vx *= 0.99;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.canvas(), frame.area());
        if !self.debug_text.is_empty() {
            let debug_text = Paragraph::new(self.debug_text.clone());
            frame.render_widget(debug_text, frame.area());
        }
    }

    fn canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .marker(self.marker)
            .paint(|ctx| {
                for ball in self.balls.iter() {
                    ctx.draw(&Points {
                        coords: &[(ball.x, ball.y)],
                        color: Color::Indexed(ball.color),
                    });
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
