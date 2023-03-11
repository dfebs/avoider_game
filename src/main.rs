use bevy::prelude::*;
use enemy::*;
use player::*;
use common_components::*;

mod common_components;
mod enemy;
mod player;

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>, // Mutable resouces wrapping a mesh asset
    mut _materials: ResMut<Assets<ColorMaterial>> // Mutable resouces wrapping a colormaterial asset
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        SpriteBundle { // dont ever query for the bundle type, aka the SpriteBundle
            sprite: Sprite { // instead of using a default, you can use a texture
                color: Color::rgb(0.1, 0.1, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.,0., 0.),
            ..default()
        },
        Velocity( Vec2 { x: 100.0 , y: 100.0 } )
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
    println!("Hello, world!");
}

// TODO
// Implement enemy spawning/despawning off-screen
// Add controller support
// Make a readme
// Look up what insert method does (it seems to be a follow up to commands.spawn())