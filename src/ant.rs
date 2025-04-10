use crate::utils::{is_quit_key, map_range};
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
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
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    is_sim_running: bool,
    pixel: bool,
    ant: (u8, usize, usize),
    speed: usize,
    n_colors: u8,
}

impl App {
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        speed: usize,
        board_width: Option<usize>,
        n_colors: u8,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let wh_factor = height / width;
        let pixel = board_width.is_none();
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

        for _ in 0..board_height {
            let mut line = Vec::new();
            line.resize(board_width, (false, 1));
            grid.push(line);
        }

        Self {
            grid,
            exit: false,
            playground: DVec2::new(width as f64, height as f64),
            marker,
            debug_text: String::new(),
            is_sim_running: true,
            pixel,
            ant: (0, board_width / 2, board_height / 2),
            speed,
            n_colors,
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
                    Event::Resize(_columns, _rows) => {
                        // self.debug_text = format!("{} {}", columns, rows);
                    }
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                for _ in 0..self.speed {
                    self.on_tick();
                }
                let board_height = self.grid.len();
                let board_width = self.grid[0].len();
                let n_touched = self.grid.iter().flatten().filter(|p| p.1 > 1).count();
                let percentage_touched = n_touched as f32 / (board_width * board_height) as f32;
                if percentage_touched > 0.9 {
                    self.reset();
                }
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.is_sim_running = false;
        for line in self.grid.iter_mut() {
            for val in line.iter_mut() {
                *val = (false, 1);
            }
        }
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('r') => self.reset(),
            _ => {
                if is_quit_key(key) {
                    self.exit = true;
                }
            }
        }
    }

    fn on_tick(&mut self) {
        if self.grid.is_empty() {
            return;
        }
        let board_height = self.grid.len();
        let board_width = self.grid[0].len();

        let (mut dir, mut x, mut y) = self.ant;
        let square_is_black = self.grid[y][x].0;
        self.grid[y][x].0 = !self.grid[y][x].0;

        let current_color = self.grid[y][x].1 as u16;

        if square_is_black {
            dir = (dir + 1) % 4;
            let new_color = (current_color + 1).clamp(0, self.n_colors as u16 - 1) as u8;
            self.grid[y][x].1 = new_color;
        } else {
            dir = (dir as i32 - 1).rem_euclid(4) as u8;
        }
        match dir {
            0 => y = (y + 1) % board_height,
            1 => x = (x + 1) % board_width,
            2 => y = (y as i32 - 1).rem_euclid(board_height as i32) as usize,
            3 => x = (x as i32 - 1).rem_euclid(board_width as i32) as usize,
            _ => unreachable!(),
        }
        self.ant = (dir, x, y);
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
