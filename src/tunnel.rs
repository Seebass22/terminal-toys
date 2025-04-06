use crate::utils::is_quit_key;
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::DVec2;
use ratatui::{
    crossterm::event::{self, Event},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::f64::consts::PI;
use std::time::{Duration, Instant};

pub struct App {
    grid: Vec<Vec<u8>>,
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    n_colors: u8,
    rotation_speed: f64,
    depth: u8,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        n_colors: u8,
        rotation_speed: f64,
        depth: u8,
    ) -> Self {
        let mut grid = Vec::new();

        let (board_width, board_height) = match marker {
            Marker::Braille => (
                (terminal_width * 2) as usize,
                (terminal_height * 4) as usize,
            ),
            _ => (terminal_width as usize, (terminal_height * 2) as usize),
        };

        for _ in 0..board_height {
            grid.push(vec![0; board_width]);
        }

        Self {
            grid,
            exit: false,
            playground: DVec2::new(board_width as f64, board_height as f64),
            marker,
            debug_text: String::new(),
            n_colors,
            rotation_speed,
            depth,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(8);
        let mut last_tick = Instant::now();
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
                self.on_tick(i);
            }
            i += 1;
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

    fn on_tick(&mut self, i: u32) {
        if self.grid.is_empty() {
            return;
        }
        let i = i as f64 * 0.03 * self.rotation_speed;
        let height = self.grid.len();
        let width = self.grid[0].len();
        let mid_y = height / 2;
        let mid_x = width / 2;
        for y in 0..height {
            for x in 0..width {
                let x2 = x as f64 - mid_x as f64;
                let y2 = y as f64 - mid_y as f64;
                let angle = y2.atan2(x2);
                let a = (PI + angle) * self.n_colors as f64 / (2.0 * PI);

                let r = match self.depth {
                    0 => 3.0 * i + (x2.powf(2.0) + y2.powf(2.0)).sqrt(),
                    1 => 3.0 * i + 20.0 * ((x2.powf(2.0) + y2.powf(2.0)).sqrt()).log2(),
                    _ => 3.0 * i + 500.0 / (x2.powf(2.0) + y2.powf(2.0)).sqrt(),
                };

                let a2 = a + i;
                let c = (a2 as u32 - (r * 0.10) as u32) % self.n_colors as u32;
                self.grid[y][x] = c as u8;
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
                for (y, line) in self.grid.iter().enumerate() {
                    for (x, color) in line.iter().enumerate() {
                        ctx.draw(&Points {
                            coords: &[(x as f64, y as f64)],
                            color: Color::Indexed(*color),
                        });
                    }
                }
            })
            .x_bounds([0.0, self.playground.x])
            .y_bounds([0.0, self.playground.y])
    }
}
