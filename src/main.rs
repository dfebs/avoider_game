use bevy::{prelude::*, sprite::collide_aabb::collide};
use common::{Velocity, CommonPlugin};
use enemy::EnemyPlugin;
use player::{Player, PlayerPlugin};
use background::*;

mod common;
mod enemy;
mod player;
mod background;

fn setup(
    mut commands: Commands,

    mut _meshes: ResMut<Assets<Mesh>>, // Mutable resouces wrapping a mesh asset
    mut _materials: ResMut<Assets<ColorMaterial>>, // Mutable resouces wrapping a colormaterial asset
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("space_ship_player.png"),
            transform: Transform::from_xyz(100.,0., 1.0),
            ..default()
        },
        Velocity( Vec2 { x: 100.0 , y: 100.0 } )
    ));
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_stars)
        .add_system(star_movement)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CommonPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}