use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Duration;
use regex::Regex;
use termion::event::Key;
use crate::debug::debug_logger::{log, log_disp};
use crate::math::euler_rotation::EulerRotation;
use crate::math::vector::Vector;
use crate::interface::input_context::InputContext;
use crate::math::quaternion::Quaternion;

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
    Move { delta: Vector },
    Rotate { delta: Quaternion },
}

#[derive(Debug, Clone, Copy)]
pub enum InterpolationMode {
    Instant,
    Linear { duration: Duration },
    Continuous,
    Oscillation { period: Duration },
}

#[derive(Debug, Copy, Clone)]
pub struct ActiveCommand {
    pub command: CommandType,
    pub interpolation: InterpolationMode,
    pub time_passed: Duration,
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
                    Ok(command) => ctx.buffer.add_command_to_obj(command, 0),
                    Err(e) => log(0, e.to_string()),
                }
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
    fn parse_command(&mut self) -> Result<ActiveCommand, String> {
        let err = Err(format!("error parsing command '{}'", self.command.clone()));
        // G1: movement type
        // G2: axis/axes
        // G3: delta
        // G4: interpolation mode
        // G5: interpolation parameter
        let regex = Regex::new(r"^([mr])([xyz]+)([\d.-]+)([cl])?([\d.-]+)?$").unwrap();
        let caps = match regex.captures(&self.command) {
            Some(caps) => caps,
            None => return err
        };

        let mut vec = Vector::zero();
        for c in caps[2].chars() {
            match c {
                'x' => vec.x = 1.0,
                'y' => vec.y = 1.0,
                'z' => vec.z = 1.0,
                _ => return err
            }
        }

        let scale = match caps[3].parse::<f32>() {
            Ok(val) => val,
            Err(_) => return err
        };

        let dur = match caps.get(5) {
            Some(s) => Duration::from_secs_f32( match s.as_str().parse::<f32>() {
                Ok(val) => val,
                Err(_) => return err
            }),
            None => Duration::from_secs(0)
        };
        let interpolation = match caps.get(4) {
            Some(i) => match i.as_str() {
                "l" => InterpolationMode::Linear { duration: dur },
                "c" => InterpolationMode::Continuous,
                _ => return err
            },
            None => InterpolationMode::Instant
        };

        let cmd = match &caps[1] {
            "m" => CommandType::Move { delta: vec * scale },
            "r" => CommandType::Rotate { delta: Quaternion::from_euler_vec(vec * scale.to_radians()) },
            _ => return err
        };

        self.add_history(self.command.clone());
        self.command.clear();

        Ok(
            ActiveCommand {
                command: cmd,
                interpolation,
                time_passed: Default::default(),
            }
        )
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command.as_str())
    }
}