use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{materials::*, meshes::*};

pub fn bullet_bundle(
    mesh: &Res<BulletMesh>,
    material: &Res<BulletMaterial>,
    transform: Transform,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: mesh.0.clone().into(),
        material: material.0.clone(),
        transform,
        ..default()
    }
}

pub fn enemy_bundle(
    mesh: &Res<EnemyMesh>,
    material: &Res<EnemyMaterial>,
    transform: Transform,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: mesh.0.clone().into(),
        material: material.0.clone(),
        transform,
        ..default()
    }
}
