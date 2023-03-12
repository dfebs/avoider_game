use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::*;
use player::*;
use common_components::*;

mod common_components;
mod enemy;
mod player;

fn detect_collisions(
    player: Query<&Transform, With<Player>>, 
    enemies: Query<&Transform, With<Enemy>>
) {
    let player_transform = player.single();
    let player_size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE);
    let enemy_size = Vec2::new(ENEMY_SIZE, ENEMY_SIZE);

    for transform in enemies.iter() {
        if let Some(_) = collide(player_transform.translation, player_size, transform.translation, enemy_size) {
            println!("OMG COLLISION");
        }
    }
}

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
        .add_system(detect_collisions)
        .run();
    println!("Hello, world!");
}

// TODO
// Add a game state for game_in_progress, and one for game_over when collision happens (bevy 0.1 has state stuff now)
// Add controller support
// Make a readme
// make the sprites
// Look up what insert method does (it seems to be a follow up to commands.spawn())
// enemies move through each other so I will want to figure out a way for them to not spawn on the same y-axis of an existing one (but only if it is slower or equal speed)
// have a thing for shooting the bad guys
// special ability where you can shoot 3 shots at the dudes, high ish cooldown 