use crate::{components::*, resources::*};
use bevy::{math::Vec3Swizzles, prelude::*};

pub fn move_system(mut query: Query<(&Move, &mut Transform), Without<Player>>, time: Res<Time>) {
    for (m, mut t) in &mut query {
        let mv_vector = t.up() * m.speed * time.delta_seconds();
        t.translation += mv_vector;
    }
}

pub fn rotate_to_player_system(
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        // get the enemy ship forward vector in 2D (already unit length)
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the dot product between the enemy forward vector and the direction to the player.
        let forward_dot_player = enemy_forward.dot(to_player);

        // if the dot product is approximately 1.0 then the enemy is already facing the player and
        // we can early out.
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // get the right vector of the enemy ship in 2D (already unit length)
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        // get the dot product of the enemy right vector and the direction to the player ship.
        // if the dot product is negative them we need to rotate counter clockwise, if it is
        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
        // with the right vector).
        let right_dot_player = enemy_right.dot(to_player);

        // determine the sign of rotation from the right dot player. We need to negate the sign
        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
        // negative is clockwise.
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        // limit rotation so we don't overshoot the target. We need to convert our dot product to
        // an angle here so we can get an angle of rotation to clamp against.
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); // clamp acos for safety

        // calculate angle of rotation with limit
        let rotation_angle = rotation_sign * (config.speed * time.delta_seconds()).min(max_angle);

        // rotate the enemy to face the player
        enemy_transform.rotate_z(rotation_angle);
    }
}

pub fn player_movement_system(
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

pub fn player_aim_system(
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
