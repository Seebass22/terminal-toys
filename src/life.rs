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
    grid: Vec<Vec<bool>>,
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    rng: Rand64,
    is_sim_running: bool,
    n_generated: usize,
    initial_n_alive: usize,
}

impl App {
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        seed: u128,
        initial_n_alive: usize,
        board_width: usize,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let rng = oorandom::Rand64::new(seed);
        let mut grid = Vec::new();

        for _ in 0..board_width {
            let mut line = Vec::new();
            line.resize(board_width, false);
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
            n_generated: 0,
            initial_n_alive,
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
                if self.is_sim_running {
                    self.on_tick();
                } else {
                    let x = self.rng.rand_range(0..self.grid.len() as u64) as usize;
                    let y = self.rng.rand_range(0..self.grid.len() as u64) as usize;
                    self.grid[y][x] = true;
                    self.n_generated += 1;
                    if self.n_generated >= self.initial_n_alive {
                        self.is_sim_running = true;
                    }
                }
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn reset(&mut self) {}

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
        if self.grid.is_empty() {
            return;
        }
        let mut new_grid = self.grid.clone();
        let height = self.grid.len();
        let width = self.grid[0].len();
        for y in 0..height {
            for x in 0..width {
                let neighbors = grid_neighbors(&self.grid, x, y);
                let n_alive = neighbors.into_iter().filter(|&val| val).count();
                if n_alive < 2 || n_alive > 3 {
                    new_grid[y][x] = false;
                }
                if n_alive == 3 {
                    new_grid[y][x] = true;
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
                let square_size = self.playground.x / width as f64;

                for (y, line) in self.grid.iter().enumerate() {
                    let y = map_range(y as f64, 0.0, width as f64, 0.0, self.playground.x);
                    for (x, &val) in line.iter().enumerate() {
                        let x = map_range(x as f64, 0.0, width as f64, 0.0, self.playground.x);
                        if val {
                            let square = Rectangle {
                                x,
                                y,
                                width: square_size,
                                height: square_size,
                                color: Color::Blue,
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

fn grid_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> [bool; 8] {
    let mut ret = [false; 8];
    let size = grid.len() as i32;
    if size == 0 {
        return ret;
    }
    for (i, (x_off, y_off)) in [
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
    .enumerate()
    {
        let mut x = x as i32 + x_off;
        let mut y = y as i32 + y_off;
        if x == -1 {
            x = size - 1;
        } else if x == size {
            x = 0;
        }
        if y == -1 {
            y = size - 1;
        } else if y == size {
            y = 0;
        }
        ret[i] = grid[y as usize][x as usize];
    }
    ret
}
