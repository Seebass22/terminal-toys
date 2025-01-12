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

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
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
    debug_text: String,
    marker: Marker,
    max_balls: u16,
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
            marker,
            debug_text: String::new(),
            max_balls: 500,
            val: 0.01,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut rng = oorandom::Rand32::new(99);
        // let p = Vec3 {
        //     x: 1.0,
        //     y: 0.0,
        //     z: 0.0,
        // };
        // let v2 = p.to_screen_position(self.playground);
        // self.debug_text = format!("{:?}", v2);
        // self.debug_text = format!("{} {}", self.playground.right(), self.playground.bottom());

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
                self.debug_text = format!("{}, {:?}", self.val, self.points);
                self.on_tick();
                last_tick = Instant::now();
                if self.tick_count % 20 == 0 && self.point_count < self.max_balls {
                    self.points.push(current_point);
                    // match self.point_count % 3 {
                    match rng.rand_range(0..6) {
                        0 => current_point.x += 1.0,
                        1 => current_point.y += 1.0,
                        2 => current_point.z += 1.0,
                        3 => current_point.x -= 1.0,
                        4 => current_point.y -= 1.0,
                        5 => current_point.z -= 1.0,
                        _ => panic!(),
                    }

                    self.point_count += 1;
                    // self.debug_text = format!("{}", self.point_count);
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
                for win in self.points.windows(2) {
                    let mut line_points: [Vec2; 2] = [Vec2 { x: 0.0, y: 0.0 }; 2];

                    for (i, point) in win.iter().enumerate() {
                        let modified_point = *point;
                        // modified_point -= model.camera_pos;
                        line_points[i] =
                            modified_point.to_screen_position(self.playground, self.val);
                    }

                    let p0 = line_points[0];
                    let p1 = line_points[1];
                    let line = Line::new(p0.x, p0.y, p1.x, p1.y, Color::Blue);
                    ctx.draw(&line);
                    // let line = Line::new(
                    //     self.playground.left() as f64,
                    //     self.playground.top() as f64,
                    //     self.playground.right() as f64,
                    //     self.playground.bottom() as f64,
                    //     Color::Blue,
                    // );
                    // ctx.draw(&line);
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
