use bevy::prelude::*;

use crate::components::{Bullet, Enemy, Health, HitCooldown, Move, Player, Sensor};

pub fn bullet_system(
    mut commands: Commands,
    q_bullets: Query<(Entity, &Transform, &Sensor), With<Bullet>>,
    q_enemies: Query<(Entity, &Transform, &Sensor), With<Enemy>>,
) {
    for (b_e, b_t, b_s) in &q_bullets {
        for (e_e, e_t, e_s) in &q_enemies {
            let mut colided = false;
            if e_t.translation.distance(b_t.translation) < (b_s.radius + e_s.radius) {
                colided = true;
                commands.entity(e_e).despawn_recursive();
                commands.entity(b_e).despawn_recursive();
            }
            if colided {
                break;
            }
        }
    }
}

pub fn player_system(
    mut commands: Commands,
    mut player: Query<(Entity, &Transform, &Sensor, &mut Health, &mut HitCooldown), With<Player>>,
    q_enemies: Query<(&Transform, &Sensor), With<Enemy>>,
    time: Res<Time>,
) {
    let (p_e, b_t, b_s, mut p_h, mut p_cd) = player.single_mut();
    if p_cd.time_remains <= 0.0 {
        for (e_t, e_s) in &q_enemies {
            if e_t.translation.distance(b_t.translation) < (b_s.radius + e_s.radius) {
                p_cd.time_remains = p_cd.time_full;
                p_h.0 -= 1.0;
                if p_h.0 <= 0.0 {
                    info!("GAME OVER!");
                    commands.entity(p_e).remove::<Move>();
                }
            }
        }
    } else {
        p_cd.time_remains -= time.delta_seconds();
    }
}
