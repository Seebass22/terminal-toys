use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
use oorandom::Rand64;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
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
    spawn_point: usize,
    color: u8,
    speed: usize,
    obstacles: usize,
    particles: u64,
    particles_spawned: usize,
    flip_after: Option<u32>,
    obstacle_len: usize,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        seed: u128,
        speed: usize,
        obstacles: usize,
        obstacle_len: usize,
        particles: u64,
        flip_after: Option<u32>,
    ) -> Self {
        let rng = oorandom::Rand64::new(seed);
        let mut grid = Vec::new();

        let (board_width, board_height) = match marker {
            Marker::HalfBlock => (terminal_width as usize, (terminal_height * 2) as usize),
            Marker::Braille => (
                (terminal_width * 2) as usize,
                (terminal_height * 4) as usize,
            ),
            _ => (terminal_width as usize, terminal_height as usize),
        };

        for _ in 0..board_height {
            let mut line = Vec::new();
            line.resize(board_width, None);
            grid.push(line);
        }

        Self {
            grid,
            exit: false,
            playground: DVec2::new(board_width as f64, board_height as f64),
            marker,
            debug_text: String::new(),
            rng,
            // spawn point and color set by initial reset()
            spawn_point: 0,
            color: 0,
            speed,
            obstacles,
            particles,
            particles_spawned: 0,
            flip_after,
            obstacle_len,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(8);
        let mut last_tick = Instant::now();
        self.reset();
        let board_width = self.grid[0].len();
        let board_height = self.grid.len();

        let mut i: u32 = 1;

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
                for _ in 0..self.speed {
                    if let Some(n) = self.flip_after {
                        if i % n == 0 {
                            self.flip();
                        }
                    }

                    self.on_tick();
                    if i % 2 == 0 {
                        self.particles_spawned += 1;
                        self.grid[0][self.spawn_point] = Some(self.color);
                        if self.rng.rand_range(0..self.particles) == 0 {
                            let width = self.grid[0].len() as u64;
                            self.spawn_point = self.rng.rand_range(0..width) as usize;
                            self.color = self.random_color();
                        }
                    }
                    i = i.wrapping_add(1);
                }
                if self.particles_spawned >= (board_height * board_width) {
                    self.reset();
                }
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.particles_spawned = 0;
        for line in self.grid.iter_mut() {
            for val in line.iter_mut() {
                *val = None;
            }
        }
        let board_width = self.grid[0].len() as u64;
        self.spawn_point = self.rng.rand_range(0..board_width) as usize;
        self.color = self.random_color();

        let bounds_x = (0, board_width);
        let board_height = self.grid.len();
        let bounds_y = (
            (board_height as f64 * 0.1) as u64,
            (board_height as f64 * 0.90) as u64,
        );
        for _ in 0..self.obstacles {
            let r = self.rng.rand_range(50..100) as f32 * 0.01;
            let mut obstacle_len = (board_width as f64 * 0.15) as i32;
            obstacle_len = (obstacle_len.min(self.obstacle_len as i32) as f32 * r) as i32;

            let x0 = self.rng.rand_range(bounds_x.0..bounds_x.1) as i32;
            let y0 = self.rng.rand_range(bounds_y.0..bounds_y.1) as i32;
            let sign = match self.rng.rand_range(0..2) {
                0 => -1,
                1 => 1,
                _ => unreachable!(),
            };
            let y_mult = match self.rng.rand_range(0..3) {
                0 => 1.0,
                1 => 0.7,
                2 => 0.0,
                _ => unreachable!(),
            };
            for i in 0..obstacle_len {
                let x = (x0 + sign * i) as usize;
                let y = (y0 + (y_mult * i as f64) as i32) as usize;
                if x >= board_width as usize || y >= board_height {
                    continue;
                }
                if let Some(1) = self.grid[y][x] {
                    break;
                }
                self.grid[y][x] = Some(1);
            }
        }
    }

    fn flip(&mut self) {
        self.grid = self.grid.clone().into_iter().rev().collect();
    }

    fn random_color(&mut self) -> u8 {
        self.rng.rand_range(2..8) as u8
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('r') => self.reset(),
            KeyCode::Char('v') => self.flip(),
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('Q') => self.exit = true,
            _ => (),
        }
    }

    fn on_tick(&mut self) {
        if self.grid.is_empty() {
            return;
        }
        let height = self.grid.len();
        let width = self.grid[0].len();
        #[allow(clippy::needless_range_loop)]
        for y in (0..(height - 1)).rev() {
            for x in 0..width {
                if self.grid[y][x].is_some() {
                    if self.grid[y][x].unwrap() == 1 {
                        continue;
                    }

                    if self.grid[y + 1][x].is_none() {
                        self.grid[y + 1][x] = self.grid[y][x];
                        self.grid[y][x] = None;
                    } else if x > 0
                        && x < (width - 1)
                        && self.grid[y + 1][x - 1].is_none()
                        && self.grid[y + 1][x + 1].is_none()
                        && self.grid[y][x - 1].is_none()
                        && self.grid[y][x + 1].is_none()
                    {
                        match self.rng.rand_range(0..2) {
                            0 => self.grid[y + 1][x - 1] = self.grid[y][x],
                            1 => self.grid[y + 1][x + 1] = self.grid[y][x],
                            _ => unreachable!(),
                        }
                        self.grid[y][x] = None;
                    } else if x > 0
                        && self.grid[y + 1][x - 1].is_none()
                        && self.grid[y][x - 1].is_none()
                    {
                        self.grid[y + 1][x - 1] = self.grid[y][x];
                        self.grid[y][x] = None;
                    } else if x < (width - 1)
                        && self.grid[y + 1][x + 1].is_none()
                        && self.grid[y][x + 1].is_none()
                    {
                        self.grid[y + 1][x + 1] = self.grid[y][x];
                        self.grid[y][x] = None;
                    }
                }
            }
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
                if self.grid.is_empty() {
                    return;
                }
                let width = self.grid[0].len();
                let height = self.grid.len();

                for (y, line) in self.grid.iter().rev().enumerate() {
                    let y = map_range(y as f64, 0.0, height as f64, 0.0, self.playground.y);
                    for (x, val) in line.iter().enumerate() {
                        let x = map_range(x as f64, 0.0, width as f64, 0.0, self.playground.x);
                        if let &Some(color) = val {
                            ctx.draw(&Points {
                                coords: &[(x, y)],
                                color: Color::Indexed(color),
                            });
                        }
                    }
                }
            })
            .x_bounds([0.0, self.playground.x])
            .y_bounds([0.0, self.playground.y])
    }
}
