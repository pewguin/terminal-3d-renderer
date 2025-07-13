use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Mutex;
use once_cell::sync::Lazy;

struct DebugLogger {
    lines: HashMap<i32, String>,
}

impl DebugLogger {
    pub fn new() -> Self {
        Self {
            lines : HashMap::new(),
        }
    }
    pub fn set_line(&mut self, line: i32, message: String) {
        self.lines.insert(line, message);
    }
}

pub static DEBUG_LOGGER : Lazy<Mutex<DebugLogger>> = Lazy::new(|| Mutex::new(DebugLogger::new()));

pub fn log<D: Debug>(line: i32, message: D) {
    let mut dbg = DEBUG_LOGGER.lock().unwrap();
    dbg.set_line(line, format!("{:?}", message));
}
pub fn log_disp<D: Display>(line: i32, message: D) {
    let mut dbg = DEBUG_LOGGER.lock().unwrap();
    dbg.set_line(line, format!("{}", message));
}
pub fn get_logs(height: u16) -> Vec<(i32, String)> {
    let dbg = DEBUG_LOGGER.lock().unwrap();
    dbg.lines.iter().map(|l| (height as i32 - *l.0, l.1.clone())).collect()
}