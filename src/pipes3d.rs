use color_eyre::Result;
use crossterm::event::KeyEventKind;
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

#[derive(Clone, Copy, Debug, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec3 {
    fn to_screen_position(self, playground: Rect, val: f64) -> Vec2 {
        // let z = self.z + 100.0;
        let z = self.z + 10.0;
        let x = (self.x) / (val * z);
        let y = (self.y) / (val * z);
        Vec2 {
            x: x + playground.right() as f64 * 0.5,
            y: y + playground.bottom() as f64 * 0.5,
        }
    }
}

pub struct App {
    exit: bool,
    points: Vec<Vec3>,
    playground: Rect,
    tick_count: u64,
    point_count: u16,
    camera_position: Vec3,
    previous_index: usize,
    debug_text: String,
    marker: Marker,
    max_segments: u16,
    val: f64,
}

impl App {
    pub fn new(terminal_width: u16, terminal_height: u16, marker: Marker) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            points: Vec::new(),
            tick_count: 0,
            point_count: 0,
            camera_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            marker,
            debug_text: String::new(),
            previous_index: 0,
            max_segments: 50000,
            val: 0.01,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut rng = oorandom::Rand32::new(99);
        let mut current_point = Vec3 {
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
                let last_point = if self.points.is_empty() {
                    Vec3::default()
                } else {
                    *self.points.last().unwrap()
                };
                let direction = last_point - self.camera_position;
                self.camera_position = self.camera_position + direction * 0.01;
                // if !self.points.is_empty() {
                //     self.debug_text = format!("{}", color_index);
                // }

                self.on_tick();
                last_tick = Instant::now();
                if self.tick_count % 2 == 0 && self.point_count < self.max_segments {
                    self.points.push(current_point);
                    let unit_vectors = [
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(-1.0, 0.0, 0.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        Vec3::new(0.0, 0.0, -1.0),
                    ];
                    let n = (self.previous_index + 3 + rng.rand_range(1..5) as usize) % 6;
                    self.previous_index = n;
                    current_point = current_point + unit_vectors[n];
                    self.point_count += 1;
                }
            }
        }
        Ok(())
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
                for (i, win) in self.points.windows(2).enumerate() {
                    let mut line_points: [Vec2; 2] = [Vec2 { x: 0.0, y: 0.0 }; 2];
                    let index_f = i as f64 * 0.1;
                    let color_index = ((index_f as u64 % 7) + 1) as u8;
                    for (i, point) in win.iter().enumerate() {
                        let modified_point = *point - self.camera_position;
                        line_points[i] =
                            modified_point.to_screen_position(self.playground, self.val);
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
