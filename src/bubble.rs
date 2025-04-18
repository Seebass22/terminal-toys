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
use std::time::{Duration, Instant};

pub struct App {
    exit: bool,
    playground: DVec2,
    debug_text: String,
    marker: Marker,
    width: usize,
    height: usize,
    elapsed_ticks: usize,
    n_colors: u8,
}

impl App {
    pub fn new(terminal_width: u16, terminal_height: u16, marker: Marker, n_colors: u8) -> Self {
        let (width, height) = match marker {
            Marker::Braille => (
                (terminal_width * 2) as usize,
                (terminal_height * 4) as usize,
            ),
            _ => (terminal_width as usize, (terminal_height * 2) as usize),
        };

        Self {
            exit: false,
            playground: DVec2::new(width as f64, height as f64),
            marker,
            debug_text: String::new(),
            width,
            height,
            elapsed_ticks: 0,
            n_colors,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(32);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_press(key),
                    Event::Resize(_columns, _rows) => {}
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                self.elapsed_ticks += 1;
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
                let n = 40;
                let m = 200;
                let r = std::f64::consts::PI * 2.0 / 235.0;

                let mut x = 0.0;
                let mut v = 0.0;
                let t = self.elapsed_ticks as f64 * 0.04;
                let size = self.height.min(self.width) as f64;

                for i in 0..n {
                    for j in 0..m {
                        let a = i as f64 + v;
                        let b = r * i as f64 + x;
                        let u = a.sin() + b.sin();
                        v = a.cos() + b.cos();
                        x = u + t;

                        let x_pos = (self.width / 2) as f64 + u * size * 0.24;
                        let y_pos = (self.height / 2) as f64 + v * size * 0.24;
                        let c = 1 + ((i % 15 + j / 36) % (self.n_colors - 1));
                        ctx.draw(&Points {
                            coords: &[(x_pos, y_pos)],
                            color: Color::Indexed(c),
                        });
                    }
                }
            })
            .x_bounds([0.0, self.playground.x])
            .y_bounds([0.0, self.playground.y])
    }
}
