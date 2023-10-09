use crate::{
    components::*,
    materials::BulletMaterial,
    meshes::BulletMesh,
    prefabs,
    resources::{MouseWorldPos, BOUNDS},
};
use bevy::prelude::*;

pub fn fire_system(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    ms_input: Res<Input<MouseButton>>,
    bullet_mat: Res<BulletMaterial>,
    bullet_mesh: Res<BulletMesh>,
) {
    if let Ok(p) = player.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) | ms_input.just_pressed(MouseButton::Left) {
            let mut b_transf = p.clone();
            b_transf.translation += b_transf.up() * 2.0;

            commands.spawn((
                prefabs::bullet_bundle(&bullet_mesh, &bullet_mat, b_transf),
                Bullet,
                Move { speed: 1000.0 },
                Decay {
                    max_seconds: 0.5,
                    elapsed_time: 0.0,
                },
                Sensor { radius: 3.0 },
            ));
        }
    }
}

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &Move)>,
    time: Res<Time>,
) {
    if let Ok((_, mut transform, mv)) = query.get_single_mut() {
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
}

pub fn aim_system(
    ms_pos: Res<MouseWorldPos>,
    q_windows: Query<&Window>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Some(_) = q_windows.single().cursor_position() {
        if let Ok(mut transf) = query.get_single_mut() {
            let displacement = ms_pos.0 - transf.translation.truncate();
            if let Some(dir) = displacement.try_normalize() {
                transf.rotation = Quat::from_rotation_arc_2d(Vec2::Y, dir);
            }
        }
    }
}

pub fn died_system(
    mut commands: Commands,
    player: Query<(Entity, &Health, &Player), Without<Dead>>,
) {
    if let Ok((e, h, _)) = player.get_single() {
        if h.0 <= 0.0 {
            info!("PLAYER DIED!");
            commands
                .entity(e)
                .insert((
                    Decay {
                        max_seconds: 3.0,
                        elapsed_time: 0.0,
                    },
                    Dead,
                ))
                .remove::<Move>();
        }
    }
}
