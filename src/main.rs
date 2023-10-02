use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, sprite::MaterialMesh2dBundle};

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement_system)
        .run()
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Move {
    speed: f32,
}

#[derive(Component)]
pub struct Rotate {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let p = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::splat(6.0)).into()).into(),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Player,
        Move { speed: 500.0 },
        Rotate { speed: f32::to_radians(360.0)},
    )).id();
    let p_child = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(2.0, 3.0)).into()).into(),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 1.0)),
            ..default()
        },
    )).id();
    commands.entity(p).add_child(p_child);
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &Move, &Rotate)>,
) {
    let (_, mut transform, mv, rot) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * rot.speed * TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * mv.speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
