use bevy::{asset::ChangeWatcher, prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Duration;

use alone::{
    diagnostics::DiagnosticsPlugin,
    prefabs::{BulletPrefab, PrefabsPlugin},
};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..Default::default()
            }),
            DiagnosticsPlugin,
            PrefabsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement_system,
                fire_system,
                decay_system,
                move_system,
            ),
        )
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

#[derive(Component)]
pub struct Decay {
    max_seconds: f32,
    elapsed_time: f32,
}

#[derive(Component)]
pub struct Bullet;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let p = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::splat(6.0)).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Player,
            Move { speed: 100.0 },
            Rotate {
                speed: f32::to_radians(360.0),
            },
        ))
        .id();
    let p_child = commands
        .spawn((MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(2.0, 3.0)).into())
                .into(),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 1.0)),
            ..default()
        },))
        .id();
    commands.entity(p).add_child(p_child);
}

fn fire_system(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    bullet_prefab: Res<BulletPrefab>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let p = player.get_single().unwrap();
        let mut b_transf = p.clone();
        b_transf.translation += b_transf.up() * 2.0;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: bullet_prefab.mesh_handle.clone().into(),
                material: bullet_prefab.material_handle.clone(),
                transform: b_transf,
                ..default()
            },
            Bullet,
            Move { speed: 1000.0 },
            Decay {
                max_seconds: 0.5,
                elapsed_time: 0.0,
            },
        ));
    }
}

fn move_system(mut query: Query<(&Move, &mut Transform), Without<Player>>, time: Res<Time>) {
    for (m, mut t) in &mut query {
        let mv_vector = t.up() * m.speed * time.delta_seconds();
        t.translation += mv_vector;
    }
}

fn decay_system(mut commands: Commands, mut query: Query<(Entity, &mut Decay)>, time: Res<Time>) {
    for (e, mut d) in &mut query {
        match d.elapsed_time > d.max_seconds {
            true => commands.entity(e).despawn_recursive(),
            false => d.elapsed_time += time.delta_seconds(),
        }
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &Move, &Rotate)>,
    time: Res<Time>,
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

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 0.5;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * rot.speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * mv.speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
