use crossterm::event::{self, KeyCode, KeyModifiers};
use glam::DVec3;
use ratatui::symbols::Marker;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn map_range(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (val - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn is_quit_key(key: event::KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => true,
        KeyCode::Char('q') => true,
        KeyCode::Char('Q') => true,
        KeyCode::Char('c') => key.modifiers == KeyModifiers::CONTROL,
        _ => false,
    }
}

// board size corresponds to marker size
// everything appears stretched if ratio is not 1:2
pub fn calc_board_size_stretched(
    marker: Marker,
    terminal_width: u16,
    terminal_height: u16,
) -> (usize, usize) {
    match marker {
        Marker::HalfBlock => (terminal_width as usize, (terminal_height * 2) as usize),
        Marker::Braille | Marker::Octant => (
            (terminal_width * 2) as usize,
            (terminal_height * 4) as usize,
        ),
        Marker::Sextant => (
            (terminal_width * 2) as usize,
            (terminal_height * 3) as usize,
        ),
        Marker::Quadrant => (
            (terminal_width * 2) as usize,
            (terminal_height * 2) as usize,
        ),
        _ => (terminal_width as usize, terminal_height as usize),
    }
}

// everything is 2x4 or 1x2
// board does not appear stretched but may look wrong
pub fn calc_board_size_scaled(
    marker: Marker,
    terminal_width: u16,
    terminal_height: u16,
) -> (usize, usize) {
    match marker {
        Marker::Braille | Marker::Octant | Marker::Quadrant | Marker::Sextant => (
            (terminal_width * 2) as usize,
            (terminal_height * 4) as usize,
        ),
        _ => (terminal_width as usize, (terminal_height * 2) as usize),
    }
}

pub trait Rotate {
    fn rotate_x(self, angle: f64) -> Self;
    fn rotate_y(self, angle: f64) -> Self;
    fn rotate_z(self, angle: f64) -> Self;
}

impl Rotate for DVec3 {
    fn rotate_z(self, angle: f64) -> DVec3 {
        let s = angle.sin();
        let c = angle.cos();
        let x = self.x * c - self.y * s;
        let y = self.x * s + self.y * c;
        DVec3::new(x, y, self.z)
    }

    fn rotate_x(self, angle: f64) -> DVec3 {
        let s = angle.sin();
        let c = angle.cos();
        let y = self.y * c - self.z * s;
        let z = self.y * s + self.z * c;
        DVec3::new(self.x, y, z)
    }

    fn rotate_y(self, angle: f64) -> DVec3 {
        let s = angle.sin();
        let c = angle.cos();
        let x = self.x * c - self.z * s;
        let z = self.x * s + self.z * c;
        DVec3::new(x, self.y, z)
    }
}
