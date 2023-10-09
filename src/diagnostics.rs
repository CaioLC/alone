use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    components::{Enemy, Health, Player},
    states::AppState,
};

#[derive(Component)]
struct TextChanges;

#[derive(Component)]
struct EnemyCounter;

#[derive(Component)]
struct PlayerHealth;

fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        TextChanges,
    ));
}

fn enemy_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Mini.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Enemies: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        EnemyCounter,
    ));
}

fn player_health(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Mini.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(35.0),
            right: Val::Px(15.0),
            ..default()
        }),
        PlayerHealth,
    ));
}

fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }
        text.sections[0].value = format!("FPS: {fps:.1} | {frame_time:.3} ms/frame",);
    }
}

fn change_enemy_counter(
    mut query: Query<&mut Text, With<EnemyCounter>>,
    q_enemies: Query<Entity, With<Enemy>>,
) {
    for mut text in &mut query {
        let total_enemies = q_enemies.iter().count();
        text.sections[1].value = format!("{total_enemies}",);
    }
}

fn player_health_update(
    mut query: Query<&mut Text, With<PlayerHealth>>,
    p_health: Query<&Health, With<Player>>,
) {
    if let Ok(health) = p_health.get_single() {
        for mut text in &mut query {
            let v = health.0;
            text.sections[1].value = format!("{v}",);
        }
    }
}

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, (infotext_system, enemy_counter, player_health))
            .add_systems(
                Update,
                (
                    change_text_system,
                    change_enemy_counter,
                    player_health_update,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
