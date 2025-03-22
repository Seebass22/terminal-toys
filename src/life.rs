use crate::utils::{calculate_hash, map_range};
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
use oorandom::Rand64;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points, Rectangle},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

pub struct App {
    grid: Vec<Vec<(bool, u8)>>,
    hash_history: Vec<u64>,
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    rng: Rand64,
    is_sim_running: bool,
    n_generated: usize,
    initial_n_alive: usize,
    pixel: bool,
}

impl App {
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        seed: u128,
        initial_percentage_alive: f32,
        board_width: Option<usize>,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let wh_factor = height / width;
        let pixel = board_width.is_none();
        let rng = oorandom::Rand64::new(seed);
        let mut grid = Vec::new();

        let (board_width, board_height) = match board_width {
            Some(width) => (width, (width as f32 * wh_factor) as usize),
            None => match marker {
                Marker::HalfBlock => (terminal_width as usize, (terminal_height * 2) as usize),
                Marker::Braille => (
                    (terminal_width * 2) as usize,
                    (terminal_height * 4) as usize,
                ),
                _ => (terminal_width as usize, terminal_height as usize),
            },
        };

        let initial_n_alive =
            (initial_percentage_alive * (board_width * board_height) as f32) as usize;

        for _ in 0..board_height {
            let mut line = Vec::new();
            line.resize(board_width, (false, 0));
            grid.push(line);
        }

        Self {
            grid,
            hash_history: Vec::new(),
            exit: false,
            playground: DVec2::new(width as f64, height as f64),
            marker,
            debug_text: String::new(),
            rng,
            is_sim_running: false,
            n_generated: 0,
            initial_n_alive,
            pixel,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(32);
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
                if self.is_sim_running {
                    self.on_tick();
                } else {
                    let ticks_to_generate = 50;
                    let n_to_generate_per_tick = self.initial_n_alive / ticks_to_generate;
                    let n_to_generate = std::cmp::min(
                        self.initial_n_alive - self.n_generated,
                        n_to_generate_per_tick,
                    );

                    for _ in 0..n_to_generate {
                        let x = self.rng.rand_range(0..self.grid[0].len() as u64) as usize;
                        let y = self.rng.rand_range(0..self.grid.len() as u64) as usize;
                        let color = self.rng.rand_range(1..13) as u8;
                        self.grid[y][x] = (true, color);
                        self.n_generated += 1;
                        if self.n_generated >= self.initial_n_alive {
                            self.is_sim_running = true;
                        }
                    }
                }
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.is_sim_running = false;
        self.n_generated = 0;
        for line in self.grid.iter_mut() {
            for val in line.iter_mut() {
                *val = (false, 0);
            }
        }
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
        for y in 0..height {
            for x in 0..width {
                let (n_alive, color) = grid_neighbors(&self.grid, x, y);
                #[allow(clippy::manual_range_contains)]
                if n_alive < 2 || n_alive > 3 {
                    new_grid[y][x] = (false, 0);
                }
                if n_alive == 3 {
                    new_grid[y][x] = (true, color);
                }
            }
        }
        self.grid = new_grid;
        let hash = calculate_hash(&self.grid);
        if self.hash_history.len() == 3 {
            if self.hash_history[0] == self.hash_history[2] {
                self.hash_history.clear();
                self.reset();
                return;
            }
            self.hash_history.rotate_left(1);
            self.hash_history.pop();
        }
        self.hash_history.push(hash);
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

                for (y, line) in self.grid.iter().enumerate() {
                    let y = map_range(y as f64, 0.0, height as f64, 0.0, self.playground.y);
                    for (x, &(val, color)) in line.iter().enumerate() {
                        let x = map_range(x as f64, 0.0, width as f64, 0.0, self.playground.x);
                        if val {
                            if self.pixel {
                                ctx.draw(&Points {
                                    coords: &[(x, y)],
                                    color: Color::Indexed(color),
                                });
                            } else {
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
                }
            })
            .x_bounds([0.0, self.playground.x])
            .y_bounds([0.0, self.playground.y])
    }
}

fn grid_neighbors(grid: &[Vec<(bool, u8)>], x: usize, y: usize) -> (usize, u8) {
    let height = grid.len() as i32;
    if height == 0 {
        return (0, 0);
    }
    let width = grid[0].len() as i32;
    let mut color = 0;
    let mut count = 0;
    for (x_off, y_off) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        //
        (-1, 0),
        (1, 0),
        //
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let mut x = x as i32 + x_off;
        let mut y = y as i32 + y_off;
        if x == -1 {
            x = width - 1;
        } else if x == width {
            x = 0;
        }
        if y == -1 {
            y = height - 1;
        } else if y == height {
            y = 0;
        }
        let item = grid[y as usize][x as usize];
        if item.0 {
            count += 1;
        }
        if color == 0 {
            color = item.1;
        }
    }
    (count, color)
}
