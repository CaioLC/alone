use bevy::prelude::*;

type MeshHandle = Handle<Mesh>;

#[derive(Resource)]
pub struct BulletMesh(pub MeshHandle);

#[derive(Resource)]
pub struct EnemyMesh(pub MeshHandle);

#[derive(Resource)]
pub struct PlayerMesh(pub MeshHandle);

pub struct MyMeshesPlugin;
impl Plugin for MyMeshesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, store_meshes);
    }
}

fn store_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let bullet_handle = meshes.add(shape::Quad::new(Vec2::new(2.0, 4.0)).into());
    commands.insert_resource(BulletMesh(bullet_handle));

    let enemy_handle = meshes.add(shape::Quad::new(Vec2::splat(6.0)).into());
    commands.insert_resource(EnemyMesh(enemy_handle));

    let player_handle = meshes.add(shape::Quad::new(Vec2::splat(6.0)).into());
    commands.insert_resource(PlayerMesh(player_handle));
}
