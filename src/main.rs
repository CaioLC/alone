use bevy::{asset::ChangeWatcher, prelude::*, sprite::MaterialMesh2dBundle, math::Vec3Swizzles};
use bevy::input::common_conditions::input_toggle_active;
use std::time::Duration;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_magic_light_2d::prelude::*;

use alone::{
    diagnostics::DiagnosticsPlugin,
    materials::{MyMaterialsPlugin, BulletMaterial},
    meshes::{MyMeshesPlugin, BulletMesh},
    prefabs,
};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Resource, Default)]
struct MouseWorldPos(pub Vec2);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseWorldPos::default())
        // .insert_resource(BevyMagicLight2DSettings {
        //     light_pass_params: LightPassParams {
        //         reservoir_size: 16,
        //         smooth_kernel_size: (2, 1),
        //         direct_light_contrib: 0.2,
        //         indirect_light_contrib: 0.8,
        //         ..default()
        //     },
        // })
        .add_plugins((
            // Bevy
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..Default::default()
            }),
            // 3rd party
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            // BevyMagicLight2DPlugin,
            // Mine
            DiagnosticsPlugin,
            MyMaterialsPlugin,
            MyMeshesPlugin
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement_system,
                player_aim_system,
                cursor_to_world,
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
    bullet_mat: Res<BulletMaterial>,
    bullet_mesh: Res<BulletMesh>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let p = player.get_single().unwrap();
        let mut b_transf = p.clone();
        b_transf.translation += b_transf.up() * 2.0;

        commands.spawn((
            prefabs::bullet_bundle(bullet_mesh, bullet_mat, b_transf),
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
    mut query: Query<(&Player, &mut Transform, &Move)>,
    time: Res<Time>,
) {
    let (_, mut transform, mv) = query.single_mut();

    let mut movement_vector = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        movement_vector += Vec2::NEG_X;
    }

    if keyboard_input.pressed(KeyCode::D) {
        movement_vector += Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement_vector += Vec2::NEG_Y;
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement_vector += Vec2::Y;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    // transform.rotate_z(rotation_factor * rot.speed * time.delta_seconds());

    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let mov = movement_vector.normalize_or_zero() * mv.speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    // let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += Vec3::new(mov.x, mov.y, 0.0);

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}


fn player_aim_system(
    ms_pos: Res<MouseWorldPos>,
    q_windows: Query<&Window>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Some(_) = q_windows.single().cursor_position() {
        let mut transf = query.get_single_mut().unwrap();
        let displacement = ms_pos.0 - transf.translation.truncate();
        if let Some(dir) = displacement.try_normalize() {
            transf.rotation = Quat::from_rotation_arc_2d(Vec2::Y, dir);
        }
    }
}


fn cursor_to_world(
    q_windows: Query<&Window>,
    query: Query<(&Camera, &GlobalTransform)>,
    ms_pos: EventReader<CursorMoved>,
    mut ms_world_pos: ResMut<MouseWorldPos>,
) {
    let window = q_windows.single();
    if let Some(cursor) = window.cursor_position() {
        if !ms_pos.is_empty() {
            let (camera, global_transf) = query.single();
            let world_pos = camera.viewport_to_world_2d(global_transf, cursor);
            if let Some(pos) = world_pos {
                ms_world_pos.0 = pos;
                // dbg!(&world_pos);
            }
        }
    }
}