use bevy::prelude::*;

type MatHandle = Handle<ColorMaterial>;

#[derive(Resource)]
pub struct BulletMaterial(pub MatHandle);

#[derive(Resource)]
pub struct FireMaterial(pub MatHandle);

pub struct MyMaterialsPlugin;
impl Plugin for MyMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, store_materials);
    }
}

fn store_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bullet_handle = materials.add(Color::RED.into());
    commands.insert_resource(BulletMaterial(bullet_handle));

    let fire_handle = materials.add(Color::ORANGE_RED.into());
    commands.insert_resource(FireMaterial(fire_handle));
}
