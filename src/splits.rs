use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
use oorandom::Rand64;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Walker {
    history: Vec<DVec2>,
    location: DVec2,
    direction: DVec2,
    active: bool,
    split_len: usize,
    color_index: u8,
}

pub struct App {
    exit: bool,
    walkers: Vec<Walker>,
    playground: Rect,
    ticks_since_stopped: u64,
    debug_text: String,
    marker: Marker,
    rng: Rand64,
    max_walkers: usize,
    rotate: bool,
}

impl App {
    pub fn new(terminal_width: u16, terminal_height: u16, marker: Marker, rotate: bool) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let first_walker = Walker {
            history: Vec::new(),
            location: DVec2::new(10.0, 0.0),
            direction: DVec2::new(0.0, 0.7),
            active: true,
            split_len: 5,
            color_index: 1,
        };
        let rng = oorandom::Rand64::new(99);
        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            walkers: vec![first_walker],
            ticks_since_stopped: 0,
            marker,
            debug_text: String::new(),
            rng,
            max_walkers: 300,
            rotate,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        self.reset();
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
                if self.ticks_since_stopped > 200 {
                    self.ticks_since_stopped = 0;
                    self.reset();
                }
                self.debug_text = format!("{}", self.walkers.len());
                self.on_tick();
                last_tick = Instant::now();
                let mut to_split = Vec::new();
                let n_walkers = self.walkers.len();
                for walker in self.walkers.iter_mut() {
                    if !walker.active {
                        continue;
                    }
                    walker.location.x += walker.direction.x;
                    walker.location.y += walker.direction.y;
                    walker.history.push(walker.location);
                    if !(self.playground.left()..=self.playground.right())
                        .contains(&(walker.location.x as u16))
                        || !(self.playground.top()..=self.playground.bottom())
                            .contains(&(walker.location.y as u16))
                    {
                        walker.active = false;
                        continue;
                    }
                    if walker.history.len() % walker.split_len == 0 && n_walkers < self.max_walkers
                    {
                        let dir = walker.direction;
                        walker.direction *= self.rng.rand_float() + 0.2;
                        walker.direction = DVec2::new(dir.y, -dir.x);
                        to_split.push(walker.clone());
                    }
                }

                for mut split_walker in to_split.into_iter() {
                    split_walker.history.clear();
                    split_walker.history.push(split_walker.location);
                    let dir = split_walker.direction;
                    split_walker.direction = DVec2::new(-dir.y, dir.x);
                    split_walker.split_len = self.rng.rand_range(20..70) as usize;
                    split_walker.color_index = (split_walker.color_index + 1) % 12;
                    self.walkers.push(split_walker);
                }
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.walkers.clear();
        let middle_x = self.playground.right() as f64 * 0.5;
        let middle_y = self.playground.bottom() as f64 * 0.5;
        let direction = if self.rotate {
            DVec2::new(self.rng.rand_float() + 0.2, self.rng.rand_float() + 0.2)
        } else {
            DVec2::new(0.0, 0.7)
        };

        let first_walker = Walker {
            history: Vec::new(),
            location: DVec2::new(middle_x, middle_y),
            direction,
            active: true,
            split_len: 5,
            color_index: 1,
        };
        self.walkers.push(first_walker);
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
        if self.walkers.len() >= self.max_walkers {
            self.ticks_since_stopped += 1;
            self.debug_text = format!("{}", self.ticks_since_stopped);
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
                for walker in self.walkers.iter() {
                    for w in walker.history.windows(2) {
                        let line_points = w;
                        let p0 = line_points[0];
                        let p1 = line_points[1];
                        let line =
                            Line::new(p0.x, p0.y, p1.x, p1.y, Color::Indexed(walker.color_index));
                        ctx.draw(&line);
                    }
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
