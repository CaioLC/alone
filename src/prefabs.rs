use bevy::prelude::*;

trait Prefab {}

#[derive(Resource, Default)]
pub struct BulletPrefab {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
}

pub struct PrefabsPlugin;

impl Plugin for PrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, store_prefabs);
    }
}

fn store_prefabs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bullet: Mesh = shape::Quad::new(Vec2::new(2.0, 4.0)).into();
    let bullet_mesh = meshes.add(bullet);
    let bullet_material = materials.add(Color::RED.into());
    commands.insert_resource(BulletPrefab {
        mesh_handle: bullet_mesh,
        material_handle: bullet_material,
    });
}
