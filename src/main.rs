use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::*;
use player::*;
use common_components::*;

mod common_components;
mod enemy;
mod player;

fn detect_collisions(
    player: Query<&Transform, With<Player>>, 
    enemies: Query<&Transform, With<Enemy>>,
    mut app_state: ResMut<State<AppState>>
) {
    let player_transform = player.single();
    let player_size = Vec2::new(PLAYER_HITBOX_SIZE, PLAYER_HITBOX_SIZE);
    let enemy_size = Vec2::new(ENEMY_HITBOX_SIZE, ENEMY_HITBOX_SIZE);

    for transform in enemies.iter() {
        if let Some(_) = collide(player_transform.translation, player_size, transform.translation, enemy_size) {
            app_state.0 = AppState::GameOver;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>, // Mutable resouces wrapping a mesh asset
    mut _materials: ResMut<Assets<ColorMaterial>>, // Mutable resouces wrapping a colormaterial asset
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        SpriteBundle { // dont ever query for the bundle type, aka the SpriteBundle
            // sprite: Sprite { // instead of using a default, you can use a texture
            //     color: Color::rgb(0.1, 0.1, 0.75),
            //     custom_size: Some(Vec2::new(50.0, 50.0)),
            //     ..default()
            // },
            texture: asset_server.load("space_ship_player.png"),
            transform: Transform::from_xyz(100.,0., 0.),
            ..default()
        },
        Velocity( Vec2 { x: 100.0 , y: 100.0 } )
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(detect_collisions)
        .run();
}