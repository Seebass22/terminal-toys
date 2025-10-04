use crate::utils::is_quit_key;
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use glam::{dvec3, DVec2, DVec3};
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

const ONE_TURN_DEGREES_F64: f64 = 360.0;

trait ConvertAngle {
    fn deg_to_rad(self) -> Self;
}

impl ConvertAngle for f64 {
    fn deg_to_rad(self) -> Self {
        self * core::f64::consts::TAU / ONE_TURN_DEGREES_F64
    }
}

trait ToScreenPos {
    fn to_screen_position(self, playgrground: Rect, val: f64) -> DVec2;
    fn to_screen_position_orthographic(self, playgrground: Rect) -> DVec2;
}

impl ToScreenPos for DVec3 {
    fn to_screen_position(self, playground: Rect, val: f64) -> DVec2 {
        let z = self.z;
        let x = (self.x) / (val * z);
        let y = (self.y) / (val * z);

        DVec2 {
            x: x + playground.right() as f64 * 0.5,
            y: y + playground.bottom() as f64 * 0.5,
        }
    }

    fn to_screen_position_orthographic(self, playground: Rect) -> DVec2 {
        let x = 3.0 * self.x + 0.4 * self.z * 3.0;
        let y = 3.0 * self.y + 0.4 * self.z * 3.0;

        DVec2 {
            x: x + playground.right() as f64 * 0.5,
            y: y + playground.bottom() as f64 * 0.5,
        }
    }
}

fn rotate_z(point: DVec3, angle: f64) -> DVec3 {
    let s = angle.sin();
    let c = angle.cos();
    let x = point.x * c - point.y * s;
    let y = point.x * s + point.y * c;
    DVec3::new(x, y, point.z)
}

fn rotate_x(point: DVec3, angle: f64) -> DVec3 {
    let s = angle.sin();
    let c = angle.cos();
    let y = point.y * c - point.z * s;
    let z = point.y * s + point.z * c;
    DVec3::new(point.x, y, z)
}

fn rotate_y(point: DVec3, angle: f64) -> DVec3 {
    let s = angle.sin();
    let c = angle.cos();
    let x = point.x * c - point.z * s;
    let z = point.x * s + point.z * c;
    DVec3::new(x, point.y, z)
}

pub struct App {
    exit: bool,
    playground: Rect,
    tick_count: u64,
    debug_text: String,
    marker: Marker,
    orthographic: bool,
    val: f64,
    points: Vec<DVec3>,
    x_rotation_speed: f64,
    y_rotation_speed: f64,
    z_rotation_speed: f64,
    amplitude: f64,
    frequency: f64,
    speed: f64,
    color_speed: Option<f64>,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        terminal_width: u16,
        terminal_height: u16,
        marker: Marker,
        orthographic: bool,
        x_rotation_speed: f64,
        y_rotation_speed: f64,
        z_rotation_speed: f64,
        amplitude: f64,
        frequency: f64,
        speed: f64,
        zoom: f64,
    ) -> Self {
        let scale_factor = terminal_height as f32 / terminal_width as f32;
        let font_scale_factor = 2.0;
        let width = 200.0;
        let height = width * scale_factor * font_scale_factor;

        let mut points = Vec::new();
        let radius = 10.0;
        for theta in (20..180).step_by(20) {
            for phi in (0..=360).step_by(10) {
                let theta = (theta as f64).deg_to_rad();
                let phi = (phi as f64).deg_to_rad();
                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.sin() * phi.sin();
                let z = radius * theta.cos();
                points.push(dvec3(x, y, z));
            }
        }

        Self {
            exit: false,
            playground: Rect::new(0, 0, width as u16, height as u16),
            tick_count: 0,
            marker,
            debug_text: String::new(),
            orthographic,
            val: -0.001 * zoom + 0.01,
            points,
            x_rotation_speed,
            y_rotation_speed,
            z_rotation_speed,
            amplitude,
            frequency,
            speed,
            color_speed: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, tick_rate: u64) -> Result<()> {
        let tick_rate = Duration::from_millis(tick_rate);
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
                last_tick = Instant::now();
                self.tick_count += 1;
            }
        }
        Ok(())
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
                let t = self.tick_count as f64 * 0.01;
                let mut c: u16 = 0;
                for win in self.points.windows(2) {
                    let mut line_points: [DVec2; 2] = [DVec2::ZERO; 2];
                    for (i, point) in win.iter().enumerate() {
                        let mut point = *point;
                        point.y += self.amplitude
                            * (self.frequency * point.z + 20.0 * self.speed * t).sin();

                        let mut modified_point = rotate_x(point, t * self.x_rotation_speed);
                        modified_point = rotate_y(modified_point, t * self.y_rotation_speed);
                        modified_point = rotate_z(modified_point, t * self.z_rotation_speed);

                        if self.orthographic {
                            line_points[i] =
                                modified_point.to_screen_position_orthographic(self.playground);
                        } else {
                            modified_point += 30.0 * DVec3::Z;
                            line_points[i] =
                                modified_point.to_screen_position(self.playground, self.val);
                        }
                    }

                    let p0 = line_points[0];
                    let p1 = line_points[1];
                    let original_p0 = win[0];
                    let original_p1 = win[1];

                    if original_p0.distance(original_p1) > 2.0 {
                        c += 1;
                        continue;
                    }
                    let mut color = c.rem_euclid(15) as u8 + 1;
                    if let Some(color_speed) = self.color_speed {
                        color += ((t * 18.0 * color_speed) as u64).rem_euclid(256) as u8;
                        color = ((color as u16).rem_euclid(256 - 16) + 16) as u8;
                    }
                    let line = Line::new(p0.x, p0.y, p1.x, p1.y, Color::Indexed(color));
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
