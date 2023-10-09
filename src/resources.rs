use bevy::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Resource, Default)]
pub struct MouseWorldPos(pub Vec2);

#[derive(Resource)]
pub struct RoundParams {
    pub round: u32,
    pub length: f32,
    pub countdown: f32,
    pub enemies: u32,
}
