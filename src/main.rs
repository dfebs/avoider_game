use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::*;
use player::*;
use common::*;

mod common;
mod enemy;
mod player;

fn detect_collisions(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>, 
    player_projectiles: Query<(Entity, &Transform), With<Projectile>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut app_state: ResMut<State<AppState>>,
    mut enemy_count: ResMut<EnemyCount>
) {
    let player_transform = player.single();

    for (entity, enemy_transform) in enemies.iter() {
        if let Some(_) = collide(player_transform.translation, PLAYER_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
            app_state.0 = AppState::GameOver;
        }

        for (projectile, projectile_transform) in player_projectiles.iter() {
            if let Some(_) = collide(projectile_transform.translation, PROJECTILE_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
                commands.entity(entity).despawn();
                commands.entity(projectile).despawn();
                enemy_count.0 -= 1;
                println!("Enemy count decremented to {}", enemy_count.0);
            }
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
        SpriteBundle {
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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system(detect_collisions)
        .add_system(projectile_movement)
        .run();
}