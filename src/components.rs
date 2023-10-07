use bevy::prelude::*;

#[derive(Component)]
pub struct Sensor {
    pub radius: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Move {
    pub speed: f32,
}

#[derive(Component)]
pub struct Rotate {
    pub speed: f32,
}

#[derive(Component)]
pub struct Decay {
    pub max_seconds: f32,
    pub elapsed_time: f32,
}

#[derive(Component)]
pub struct Bullet;
