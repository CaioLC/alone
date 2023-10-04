use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{meshes::BulletMesh, materials::BulletMaterial};

pub fn bullet_bundle(
    bullet_mesh: Res<BulletMesh>,
    bullet_material: Res<BulletMaterial>,
    transform: Transform
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: bullet_mesh.0.clone().into(),
        material: bullet_material.0.clone(),
        transform,
        ..default()
    }
}
