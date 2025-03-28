use crossterm::event::{self, KeyCode, KeyModifiers};
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
