use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Duration;
use termion::event::Key;
use crate::debug::debug_logger::{log, log_disp};
use crate::math::euler_rotation::EulerRotation;
use crate::math::vector::Vector;
use crate::interface::input_context::InputContext;
use crate::math::quaternion::Quaternion;

const AXIS_LETTERS: [char; 3] = ['x', 'y', 'z'];
const RECOGNIZED_COMMANDS: [char; 3] = [ 'm', 'r', 's'];

enum Command {
    Move { delta: Vector, mode: InterpolationMode },
    Rotate { delta: Quaternion, mode: InterpolationMode },
}

enum InterpolationMode {
    Instant,
    Linear { duration: Duration },
    Continuous,
    Oscillation { period: Duration },
}

pub struct ActiveCommand {
    command: Command,
    time: Duration,
}

#[derive(Debug)]
pub struct Input {
    command: String,
    history: VecDeque<String>,
    current_history: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            command: String::with_capacity(16),
            history: VecDeque::with_capacity(20),
            current_history: 0,
        }
    }
    pub fn process_key(&mut self, k: Key, ctx: &mut InputContext) {
        if let Key::Char(c) = k {
            if c == '\n' {
                match self.parse_command() {
                    Ok(cmd) => {
                        match cmd {
                            Command::Move { delta, mode } => {
                                match mode {
                                    InterpolationMode::Instant => *ctx.mesh = ctx.mesh.translate(&delta),
                                    _ => {}
                                }
                            }
                            Command::Rotate { delta, mode } => {
                                match mode {
                                    InterpolationMode::Instant => *ctx.mesh = ctx.mesh.rotate(&delta),
                                    _ => {}
                                }
                            }
                        }
                    }
                    Err(e) => log_disp(5, e)
                }
                self.command.clear();
            }
            else {
                self.command.push(c);
            }
        }
        else if let Key::Ctrl(c) = k {
            if c == 'c' {
                *ctx.exit = true
            }
        }
        else if matches!(k, Key::Backspace) {
            self.command.pop();
        }
        else if matches!(k, Key::Up) {
            if self.history.len() == 0 {
                return;
            }
            if self.current_history < self.history.len() {
                self.current_history += 1;
            }
            self.command = self.get_history(self.current_history);
        }
        else if matches!(k, Key::Down) {
            self.current_history = self.current_history.max(1) - 1;
            if self.current_history == 0 {
                self.command = String::new();
            }
            else {
                self.command = self.get_history(self.current_history);
            }
        }
    }
    fn add_history(&mut self, line: String) {
        if self.history.len() > self.history.capacity() {
            self.history.pop_back();
        }
        self.history.push_front(line);
    }
    fn get_history(&self, i: usize) -> String {
        self.history.get(i.max(1) - 1).unwrap().clone()
    }
    fn parse_command(&mut self) -> Result<Command, String> {
        let cmd: Command;
        let cmd_str = self.command.clone();
        let pfx = self.command.remove(0);
        if !RECOGNIZED_COMMANDS.contains(&pfx) {
            return Err("command not recognized".to_string());
        }
        let mut c = match self.command.chars().nth(0) {
            Some(c) => c,
            None => return Err("missing axis".to_string()),
        };
        let mut delta_vec = Vector::zero();
        while AXIS_LETTERS.contains(&c) {
            self.command.remove(0);
            match c {
                'x' => delta_vec.x += 1.0,
                'y' => delta_vec.y += 1.0,
                'z' => delta_vec.z += 1.0,
                _ => {}
            }
            c = self.command.chars().nth(0).unwrap();
        };
        
        let rtr = Ok(match self.command.parse::<f32>() {
            Ok(v) => {
                match pfx {
                    'm' => Command::Move { delta: delta_vec * v, mode: InterpolationMode::Instant },
                    'r' => Command::Rotate { delta: Quaternion::from_euler_vec(delta_vec * v.to_radians()), mode: InterpolationMode::Instant },
                    _ => panic!("theoretically impossible to get this error message.")
                }
                
            }
            Err(e) => return Err(format!("could not parse delta, read as {}", self.command)),
        });
        self.add_history(cmd_str);
        self.command.clear();
        rtr
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command.as_str())
    }
}