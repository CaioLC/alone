use bevy::prelude::*;

type MatHandle = Handle<ColorMaterial>;

#[derive(Resource)]
pub struct BulletMaterial(pub MatHandle);

#[derive(Resource)]
pub struct PlayerMaterial(pub MatHandle);

#[derive(Resource)]
pub struct EnemyMaterial(pub MatHandle);

pub struct MyMaterialsPlugin;
impl Plugin for MyMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, store_materials);
    }
}

fn store_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let bullet_handle = materials.add(Color::ORANGE.into());
    commands.insert_resource(BulletMaterial(bullet_handle));

    let enemy_handle = materials.add(Color::RED.into());
    commands.insert_resource(EnemyMaterial(enemy_handle));

    let player_handle = materials.add(Color::WHITE.into());
    commands.insert_resource(EnemyMaterial(player_handle));
}
