use alone::materials::EnemyMaterial;
use alone::meshes::EnemyMesh;
use alone::states::{AppState, StatesPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::{asset::ChangeWatcher, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;
use std::time::Duration;
// use bevy_magic_light_2d::prelude::*;

use alone::{
    components::*,
    diagnostics::DiagnosticsPlugin,
    materials::MyMaterialsPlugin,
    meshes::MyMeshesPlugin,
    prefabs,
    resources::*,
    systems::{collision, movement, player},
    ui::UIPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseWorldPos::default())
        .add_plugins((
            // Bevy
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..Default::default()
            }),
            // 3rd party
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            // Mine
            StatesPlugin,
            DiagnosticsPlugin,
            MyMaterialsPlugin,
            MyMeshesPlugin,
            UIPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(
            Update,
            (
                cursor_to_world,
                decay_system,
                enemy_system,
                end_game,
                movement::rotate_to_player_system,
                collision::bullet_enemy,
                collision::player_enemy,
                player::movement_system,
                player::aim_system,
                player::fire_system,
                player::died_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, movement::move_system)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Enemy>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
    spawn_player(&mut commands, meshes, materials);
    commands.insert_resource(RoundParams {
        round: 1,
        length: 10.0,
        countdown: 10.0,
        enemies: 10,
    });
}

fn spawn_player(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let p = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::splat(6.0)).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Player,
            Health(5.0),
            HitCooldown {
                time_full: 2.0,
                time_remains: 0.0,
            },
            Move { speed: 100.0 },
            Sensor { radius: 5.0 },
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

fn decay_system(mut commands: Commands, mut query: Query<(Entity, &mut Decay)>, time: Res<Time>) {
    for (e, mut d) in &mut query {
        match d.elapsed_time > d.max_seconds {
            true => commands.entity(e).despawn_recursive(),
            false => d.elapsed_time += time.delta_seconds(),
        }
    }
}

fn end_game(
    mut next_state: ResMut<NextState<AppState>>,
    p: Query<&Player>
) {
    if p.get_single().is_err() {
        next_state.set(AppState::GameOver)
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

fn enemy_system(
    mut commands: Commands,
    mut round: ResMut<RoundParams>,
    time: Res<Time>,
    enemy_mesh: Res<EnemyMesh>,
    enemy_mat: Res<EnemyMaterial>,
) {
    if round.length == round.countdown {
        for _ in 0..round.enemies {
            let random_pos = random_2d((-600.0, 600.0), (-300.0, 300.0));
            let mut t = Transform::from_xyz(random_pos.x, random_pos.y, 0.0);
            t.rotate_z(rand::random::<f32>() * 360.0);
            commands.spawn((
                prefabs::enemy_bundle(&enemy_mesh, &enemy_mat, t),
                Enemy,
                Move { speed: 50.0 },
                RotateToPlayer { speed: 180.0 },
                Sensor { radius: 7.0 },
            ));
        }
    }
    round.countdown -= time.delta_seconds();
    if round.countdown <= 0.0 {
        round.countdown = round.length;
        round.enemies = (round.enemies as f32 * 1.2).ceil() as u32;
    }
}

type MinMax = (f32, f32);
fn random_2d(x_range: MinMax, y_range: MinMax) -> Vec2 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(x_range.0..x_range.1);
    let y: f32 = rng.gen_range(y_range.0..y_range.1);
    Vec2::new(x, y)
}
