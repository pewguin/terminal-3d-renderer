use std::time::Duration;
use crate::debug::debug_logger::log;
use crate::interface::input::{ActiveCommand, CommandType, InterpolationMode};
use crate::math::euler_rotation::EulerRotation;
use crate::math::mesh::Mesh;
use crate::math::quaternion::Quaternion;
use crate::math::triangle::Triangle;
use crate::math::vector::Vector;

#[derive(Clone, Debug)]
pub struct Object {
    pub base_mesh: Mesh,
    pub active_commands: Vec<ActiveCommand>,
}

impl Object {
    pub fn new(m: Mesh) -> Object {
        Object {
            base_mesh: m,
            active_commands: Vec::new(),
        }
    }
    pub fn add_command(&mut self, command: ActiveCommand) {
        self.active_commands.push(command);
    }
    // The time that passes tells me many lovely things
    pub fn pass_time(&mut self, time: Duration) {
        for cmd in &mut self.active_commands {
            cmd.time_passed += time;
        }
    }
    pub fn apply_commands(&mut self) -> Mesh {
        let mut total_translation = Vector::zero();
        let mut total_rotation = Quaternion::identity();
        let mut over = false;

        for cmd in &self.active_commands {
            let mut translation = Vector::zero();
            let mut rotation = Quaternion::identity();

            match cmd.command {
                CommandType::Move { delta } => translation += delta,
                CommandType::Rotate { delta } => rotation = rotation * delta,
            }

            let coef: f32 = match cmd.interpolation {
                InterpolationMode::Instant => {
                    over = true;
                    1.0
                },
                InterpolationMode::Linear { duration } => {
                    over = cmd.time_passed > duration;
                    cmd.time_passed.as_secs_f32() / duration.as_secs_f32()
                },
                InterpolationMode::Continuous => cmd.time_passed.as_secs_f32(),
                InterpolationMode::Oscillation { period } => todo!()
            };

            translation *= coef;
            rotation = Quaternion::identity().slerp(rotation, coef);

            if over {
                self.base_mesh = self.base_mesh.translate(&translation).rotate(&rotation);
            }
            else {
                total_translation += translation;
                total_rotation = total_rotation * rotation;
            }
        }

        self.active_commands.retain(|cmd| {
           match cmd.interpolation {
               InterpolationMode::Instant => false,
               InterpolationMode::Continuous => true,
               InterpolationMode::Linear { duration } => cmd.time_passed.as_secs_f32() > duration.as_secs_f32(),
               InterpolationMode::Oscillation { .. } => true,
           }
        });

        self.base_mesh.clone().translate(&total_translation).rotate(&total_rotation)
    }
}