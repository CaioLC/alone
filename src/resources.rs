use bevy::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Resource, Default)]
pub struct MouseWorldPos(pub Vec2);
