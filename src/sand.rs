use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
use oorandom::Rand64;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Rectangle},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

pub fn map_range(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (val - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub struct App {
    grid: Vec<Vec<Option<u8>>>,
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    rng: Rand64,
    is_sim_running: bool,
    spawn_point: usize,
    color: u8,
}

impl App {
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        seed: u128,
        board_width: usize,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let rng = oorandom::Rand64::new(seed);
        let mut grid = Vec::new();

        let wh_factor = height / width;
        let board_height = (board_width as f32 * wh_factor) as usize;

        for _ in 0..board_height {
            let mut line = Vec::new();
            line.resize(board_width, None);
            grid.push(line);
        }

        Self {
            grid,
            exit: false,
            playground: DVec2::new(width as f64, height as f64),
            marker,
            debug_text: String::new(),
            rng,
            is_sim_running: false,
            spawn_point: 2,
            color: 1,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(8);
        let mut last_tick = Instant::now();
        self.reset();

        let mut i = 0;
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
                last_tick = Instant::now();
                self.on_tick();
                if i % 2 == 0 {
                    self.grid[0][self.spawn_point] = Some(self.color);
                    if self.rng.rand_range(0..100) == 0 {
                        let width = self.grid[0].len() as u64;
                        self.spawn_point = self.rng.rand_range(0..width) as usize;
                        self.color = self.rng.rand_range(1..13) as u8;
                    }
                }
                i += 1;
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.is_sim_running = false;
        for line in self.grid.iter_mut() {
            for val in line.iter_mut() {
                *val = None;
            }
        }
        let width = self.grid[0].len() as u64;
        self.spawn_point = self.rng.rand_range(0..width) as usize;
        self.color = self.rng.rand_range(1..13) as u8;
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('r') => self.reset(),
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('Q') => self.exit = true,
            _ => (),
        }
    }

    fn on_tick(&mut self) {
        if self.grid.is_empty() {
            return;
        }
        let mut new_grid = self.grid.clone();
        let height = self.grid.len();
        let width = self.grid[0].len();
        #[allow(clippy::needless_range_loop)]
        for y in 0..(height - 1) {
            for x in 0..width {
                if self.grid[y][x].is_some() {
                    if self.grid[y + 1][x].is_none() {
                        new_grid[y + 1][x] = self.grid[y][x];
                        new_grid[y][x] = None;
                    } else if x > 0
                        && x < (width - 1)
                        && self.grid[y + 1][x - 1].is_none()
                        && self.grid[y + 1][x + 1].is_none()
                    {
                        match self.rng.rand_range(0..2) {
                            0 => new_grid[y + 1][x - 1] = self.grid[y][x],
                            1 => new_grid[y + 1][x + 1] = self.grid[y][x],
                            _ => unreachable!(),
                        }
                        new_grid[y][x] = None;
                    } else if x > 0 && self.grid[y + 1][x - 1].is_none() {
                        new_grid[y + 1][x - 1] = self.grid[y][x];
                        new_grid[y][x] = None;
                    } else if x < (width - 1) && self.grid[y + 1][x + 1].is_none() {
                        new_grid[y + 1][x + 1] = self.grid[y][x];
                        new_grid[y][x] = None;
                    }
                }
            }
        }
        self.grid = new_grid;
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
                if self.grid.is_empty() {
                    return;
                }
                let width = self.grid[0].len();
                let height = self.grid.len();
                let square_width = self.playground.x / width as f64;
                let square_height = self.playground.y / height as f64;

                for (y, line) in self.grid.iter().rev().enumerate() {
                    let y = map_range(y as f64, 0.0, height as f64, 0.0, self.playground.y);
                    for (x, val) in line.iter().enumerate() {
                        let x = map_range(x as f64, 0.0, width as f64, 0.0, self.playground.x);
                        if let &Some(color) = val {
                            let square = Rectangle {
                                x,
                                y,
                                width: square_width,
                                height: square_height,
                                color: Color::Indexed(color),
                            };
                            ctx.draw(&square);
                        }
                    }
                }
            })
            .x_bounds([0.0, self.playground.x])
            .y_bounds([0.0, self.playground.y])
    }
}
