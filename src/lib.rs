use std::f32::consts::PI;

pub mod engine;
pub use engine::*;

pub mod object;
pub use object::Object;

pub mod wrappers;
pub use wrappers::*;

pub fn radians(angle: f32) -> f32 {
    PI * angle / 180.
}
