use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::{DVec2, DVec3};
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

trait ToScreenPos {
    fn to_screen_position(self, playgrground: Rect, val: f64) -> DVec2;
    fn to_screen_position_orthographic(self, playgrground: Rect) -> DVec2;
}

impl ToScreenPos for DVec3 {
    fn to_screen_position(self, playground: Rect, val: f64) -> DVec2 {
        let z = self.z + 10.0;
        let x = (self.x) / (val * z);
        let y = (self.y) / (val * z);

        DVec2 {
            x: x + playground.right() as f64 * 0.5,
            y: y + playground.bottom() as f64 * 0.5,
        }
    }

    fn to_screen_position_orthographic(self, playground: Rect) -> DVec2 {
        let x = 20.0 * self.x + 0.4 * self.z * 20.0;
        let y = 20.0 * self.y + 0.4 * self.z * 20.0;

        DVec2 {
            x: x + playground.right() as f64 * 0.5,
            y: y + playground.bottom() as f64 * 0.5,
        }
    }
}

pub struct App {
    exit: bool,
    points: Vec<DVec3>,
    playground: Rect,
    tick_count: u64,
    camera_position: DVec3,
    previous_index: usize,
    debug_text: String,
    marker: Marker,
    max_segments: u32,
    orthographic: bool,
    val: f64,
}

impl App {
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        max_segments: u32,
        orthographic: bool,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            points: Vec::with_capacity(max_segments as usize),
            tick_count: 0,
            camera_position: DVec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            marker,
            debug_text: String::new(),
            previous_index: 0,
            max_segments,
            orthographic,
            val: 0.01,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, tick_rate: u64, seed: u64) -> Result<()> {
        let tick_rate = Duration::from_millis(tick_rate);
        let mut last_tick = Instant::now();
        let mut rng = oorandom::Rand32::new(seed);
        let mut current_point = DVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
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
                if self.points.len() as u32 >= self.max_segments {
                    self.reset();
                }
                let last_point = if self.points.is_empty() {
                    DVec3::default()
                } else {
                    *self.points.last().unwrap()
                };
                let direction = last_point - self.camera_position;
                self.camera_position += direction * 0.01;
                self.on_tick();
                last_tick = Instant::now();
                if self.tick_count % 2 == 0 && (self.points.len() as u32) < self.max_segments {
                    self.points.push(current_point);
                    let unit_vectors = [
                        DVec3::new(1.0, 0.0, 0.0),
                        DVec3::new(0.0, 1.0, 0.0),
                        DVec3::new(0.0, 0.0, 1.0),
                        DVec3::new(-1.0, 0.0, 0.0),
                        DVec3::new(0.0, -1.0, 0.0),
                        DVec3::new(0.0, 0.0, -1.0),
                    ];
                    let n = (self.previous_index + 3 + rng.rand_range(1..5) as usize) % 6;
                    self.previous_index = n;
                    current_point += unit_vectors[n];
                }
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.points.clear();
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('Q') => self.exit = true,
            KeyCode::Char('a') => self.val += 0.001,
            KeyCode::Char('d') => self.val -= 0.001,
            _ => (),
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
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
                'outer: for (i, win) in self.points.windows(2).enumerate() {
                    let mut line_points: [DVec2; 2] = [DVec2::ZERO; 2];
                    let index_f = i as f64 * 0.1;
                    let color_index = ((index_f as u64 % 7) + 1) as u8;
                    for (i, point) in win.iter().enumerate() {
                        let modified_point = *point - self.camera_position;
                        if modified_point.z < -9.0 {
                            continue 'outer;
                        }
                        if self.orthographic {
                            line_points[i] =
                                modified_point.to_screen_position_orthographic(self.playground);
                        } else {
                            line_points[i] =
                                modified_point.to_screen_position(self.playground, self.val);
                        }
                    }

                    let p0 = line_points[0];
                    let p1 = line_points[1];
                    let line = Line::new(p0.x, p0.y, p1.x, p1.y, Color::Indexed(color_index));
                    ctx.draw(&line);
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
