use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;
use crate::{common::*, stage_manager::CurrentStage};

#[derive(Component)]
pub enum Enemy {
    Standard
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
pub struct EnemyCount(pub i32);

pub const ENEMY_HITBOX: Vec2 = Vec2::new(34.0, 54.0); // Enemy sprite is 64x64, this is more lenient 
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_systems(
            (enemy_spawning, enemy_movement, check_for_stage_update).in_set(OnUpdate(AppState::InGame))
        );
    }
}

fn enemy_spawning( // TODO make random enemy based on enemy type that is expressed by the level
    mut commands: Commands, 
    window: Query<&Window>,
    time: Res<Time>, 
    mut timer: ResMut<EnemySpawnTimer>,
    mut enemy_count: ResMut<EnemyCount>,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        let window = window.single();
        commands.spawn((
            Enemy::Standard,
            SpriteBundle {
                texture: asset_server.load("space_ship_enemy.png"),
                transform: Transform::from_xyz(window.width() / 2.0 + ENEMY_HITBOX.x, rng.gen_range(-300.0..300.0), 1.0),
                ..default()
            },
            Velocity( Vec2 { x: rng.gen_range(-400.0..-300.0) , y: 0.0 } )
        ));

        enemy_count.0 += 1;
    }
}

fn check_for_stage_update(
    current_stage: Res<CurrentStage>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.0.duration() == Duration::from_secs_f32(current_stage.0.enemy_spawn_rate_sec) {
        return;
    }

    println!("Enemy spawn rate changed to {}", current_stage.0.enemy_spawn_rate_sec);

    enemy_spawn_timer.0 = Timer::from_seconds(current_stage.0.enemy_spawn_rate_sec, TimerMode::Repeating)
}

pub fn enemy_movement(
    mut commands: Commands, 
    window: Query<&Window>, 
    mut enemies: Query<(Entity, &Velocity, &mut Transform), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    time: Res<Time>
) {
    let mut rng = rand::thread_rng();
    let window = window.single();
    for (entity, vel, mut transform) in enemies.iter_mut() {
        transform.translation.x += vel.0.x * time.delta_seconds();
        if transform.translation.x < -window.width() / 2.0 - ENEMY_HITBOX.x { 
            transform.translation.x = window.width() / 2.0 + ENEMY_HITBOX.x;
            transform.translation.y = rng.gen_range(-300.0..300.0);
        }
    }

}