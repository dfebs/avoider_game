use bevy::prelude::*;
use rand::Rng;
use crate::common_components::*;


#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
pub struct EnemyCount(i32);

pub const ENEMY_SIZE : f32 = 50.0;
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_system(enemy_spawning)
        .add_system(enemy_movement);
    }
}

fn enemy_spawning(
    mut commands: Commands, 
    window: Query<&Window>,
    time: Res<Time>, 
    mut timer: ResMut<EnemySpawnTimer>,
    mut enemy_count: ResMut<EnemyCount>
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        let window = window.single();
        commands.spawn((
            Enemy,
            SpriteBundle { // dont ever query for the bundle type, aka the SpriteBundle
                sprite: Sprite { // instead of using a default, you can use a texture
                    color: Color::rgb(0.9, 0.1, 0.2),
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(window.width() / 2.0 + ENEMY_SIZE, rng.gen_range(-300.0..300.0), 0.),
                ..default()
            },
            Velocity( Vec2 { x: rng.gen_range(-100.0..-10.0) , y: 0.0 } )
        ));

        enemy_count.0 += 1;
        println!("enemy_count incremented to {}", enemy_count.0);
    }
}

pub fn enemy_movement(
    mut commands: Commands, 
    window: Query<&Window>, 
    mut enemies: Query<(Entity, &Velocity, &mut Transform), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    time: Res<Time>
) {
    let window = window.single();
    for (entity, vel, mut transform) in enemies.iter_mut() {
        transform.translation.x += vel.0.x * time.delta_seconds();
        if transform.translation.x < -window.width() / 2.0 - ENEMY_SIZE { 
            commands.entity(entity).despawn();
            enemy_count.0 -= 1;
            println!("enemy count decremented to {}", enemy_count.0);
        }
    }

}