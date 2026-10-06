use crate::utils::{is_quit_key, map_range};
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
    points: Vec<Vec<DVec3>>,
    playground: Rect,
    tick_count: u64,
    camera_position: DVec3,
    previous_index: Vec<usize>,
    debug_text: String,
    marker: Marker,
    max_segments: u32,
    orthographic: bool,
    val: f64,
    rotate: bool,
    fixed: bool,
    boundary: f64,
    current_rotation: f64,
    n_lines: usize,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        max_segments: u32,
        orthographic: bool,
        rotate: bool,
        boundary: u64,
        fixed: bool,
        n_lines: usize,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;
        let mut points = Vec::new();
        let mut previous_index = Vec::new();
        for _ in 0..n_lines {
            points.push(Vec::with_capacity(max_segments as usize));
            previous_index.push(0);
        }

        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            points,
            tick_count: 0,
            camera_position: DVec3::default(),
            marker,
            debug_text: String::new(),
            previous_index,
            max_segments,
            orthographic,
            val: 0.01,
            rotate,
            fixed,
            boundary: boundary as f64,
            current_rotation: 0.0,
            n_lines,
        }
    }

    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        tick_rate: u64,
        seed: u64,
        camera_speed: f64,
    ) -> Result<()> {
        let tick_rate = Duration::from_millis(tick_rate);
        let mut last_tick = Instant::now();
        let mut rng = oorandom::Rand32::new(seed);
        let follow_speed = map_range(camera_speed, 0.0, 10.0, 0.0, 0.01).clamp(0.0, 1.0);
        let unit_vectors = [
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(-1.0, 0.0, 0.0),
            DVec3::new(0.0, -1.0, 0.0),
            DVec3::new(0.0, 0.0, -1.0),
        ];
        if self.fixed {
            self.current_rotation = random_rotation(&mut rng);
        }
        let mut current_point = Vec::new();
        for _ in 0..self.n_lines {
            current_point.push(DVec3::ZERO);
        }

        while !self.exit {
            // if let Some(current_point) = self.points.iter().last() {
            //     self.debug_text = format!("{}", current_point);
            // }
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
                self.on_tick();

                if self.points[0].len() as u32 >= self.max_segments && !self.rotate {
                    self.reset();
                    for p in current_point.iter_mut() {
                        *p = DVec3::ZERO;
                    }
                    self.current_rotation = random_rotation(&mut rng);
                }

                for (i, points) in self.points.iter_mut().enumerate() {
                    if points.len() as u32 >= self.max_segments && self.rotate {
                        points.rotate_left(1);
                        points.pop();
                    }
                    let last_point = if points.is_empty() {
                        DVec3::default()
                    } else {
                        *points.last().unwrap()
                    };

                    let direction = last_point - self.camera_position;
                    self.camera_position += direction * follow_speed;
                    last_tick = Instant::now();
                    if self.tick_count.is_multiple_of(2)
                        && (points.len() as u32) < self.max_segments
                    {
                        points.push(current_point[i]);
                        let mut next_point = DVec3::new(100000000.0, 0.0, 0.0);
                        let mut n = 0;
                        while next_point.x.abs() > self.boundary
                            || next_point.y.abs() > self.boundary
                            || next_point.z.abs() > self.boundary
                        {
                            n = (self.previous_index[i] + 3 + rng.rand_range(1..5) as usize) % 6;
                            next_point = current_point[i] + unit_vectors[n];
                        }
                        self.previous_index[i] = n;
                        current_point[i] = next_point;
                    }
                }
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        for points in self.points.iter_mut() {
            points.clear();
        }
        self.camera_position = DVec3::default();
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('a') => self.val += 0.001,
            KeyCode::Char('d') => self.val -= 0.001,
            _ => {
                if is_quit_key(key) {
                    self.exit = true;
                }
            }
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
                for (c, points) in self.points.iter().enumerate() {
                    'outer: for (i, win) in points.windows(2).enumerate() {
                        let mut line_points: [DVec2; 2] = [DVec2::ZERO; 2];
                        let index_f = i as f64 * 0.1;

                        let mut color_index = ((index_f as u64 % 7) + 1) as u8;
                        if self.n_lines > 1 {
                            color_index = 1 + c as u8;
                        }

                        for (i, point) in win.iter().enumerate() {
                            let mut modified_point = *point;
                            if self.fixed {
                                modified_point = rotate_y(*point, self.current_rotation);
                            } else {
                                modified_point -= self.camera_position;
                            }
                            if modified_point.z < -9.0 && !self.orthographic {
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

fn rotate_y(point: DVec3, angle: f64) -> DVec3 {
    let s = angle.sin();
    let c = angle.cos();
    let x = point.x * c - point.z * s;
    let z = point.x * s + point.z * c;
    DVec3::new(x, point.y, z)
}

fn random_rotation(rng: &mut oorandom::Rand32) -> f64 {
    -0.5 * std::f64::consts::PI + (rng.rand_float() * std::f32::consts::PI) as f64
}
