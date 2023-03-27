use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;
use crate::{common::*, stage_manager::CurrentStage};

#[derive(Component, Copy, Clone)]
pub enum Enemy {
    Standard,
    Wavy(f32)
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

fn spawn_enemy(
    mut commands: Commands, 
    window: Query<&Window>,
    mut enemy_count: ResMut<EnemyCount>,
    asset_server: Res<AssetServer>,
    enemy_type: Enemy
) {
    let mut rng = rand::thread_rng();

    let window = window.single();
    let starting_x_pos = window.width() / 2.0 + ENEMY_HITBOX.x;
    let starting_x_velocity = rng.gen_range(-400.0..-300.0);
    let mut starting_y_pos;

    match enemy_type {
        Enemy::Wavy(wave_size) => { // assume wave_size of 1 or 2
            starting_y_pos = rng.gen_range(-200.0..200.0); // this WILL need to change, it will be based on wave_size
            commands.spawn((
                enemy_type,
                SpriteBundle {
                    texture: asset_server.load("space_ship_enemy.png"),
                    transform: Transform::from_xyz(starting_x_pos, starting_y_pos, 1.0),
                    ..default()
                },
                Velocity( Vec2 { x: starting_x_velocity , y: 0.0 } ),
            ));
        }
        Enemy::Standard => {
            let starting_y_pos = rng.gen_range(-300.0..300.0); // This should change to not be hardcoded
            commands.spawn((
                enemy_type,
                SpriteBundle {
                    texture: asset_server.load("space_ship_enemy.png"),
                    transform: Transform::from_xyz(starting_x_pos, starting_y_pos, 1.0),
                    ..default()
                },
                Velocity( Vec2 { x: starting_x_velocity , y: 0.0 } )
            ));
        }
    }

    enemy_count.0 += 1;
}

fn enemy_spawning(
    commands: Commands, 
    window: Query<&Window>,
    time: Res<Time>, 
    mut timer: ResMut<EnemySpawnTimer>,
    enemy_count: ResMut<EnemyCount>,
    asset_server: Res<AssetServer>,
    current_stage: Res<CurrentStage>,
) {

    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let enemy_types = &current_stage.0.enemy_types;
    let enemy_type = enemy_types[rng.gen_range(0..enemy_types.len())];

    spawn_enemy(commands, window, enemy_count, asset_server, enemy_type);
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
    mut enemies: Query<(Entity, &mut Velocity, &mut Transform, &Enemy)>,
    mut enemy_count: ResMut<EnemyCount>,
    time: Res<Time>
) {
    let mut rng = rand::thread_rng();
    let window = window.single();
    for (entity, mut vel, mut transform, enemy_type) in enemies.iter_mut() {
        let delta = time.delta_seconds();
        transform.translation.x += vel.0.x * delta;
        transform.translation.y += vel.0.y * delta;
        if transform.translation.x < -window.width() / 2.0 - ENEMY_HITBOX.x { 
            transform.translation.x = window.width() / 2.0 + ENEMY_HITBOX.x;
            transform.translation.y = rng.gen_range(-300.0..300.0);
        }

        match enemy_type {
            Enemy::Wavy(wave_size) => {
                vel.0.y = f32::sin(transform.translation.x / 50.0) * 500.0 * wave_size;
            }
            Enemy::Standard => {}
        }
    }

}

