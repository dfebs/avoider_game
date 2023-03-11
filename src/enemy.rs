use bevy::prelude::*;
use rand::Rng;
use crate::common_components::*;


#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
pub struct EnemyCount(i32);

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
    time: Res<Time>, 
    mut timer: ResMut<EnemySpawnTimer>,
    mut enemy_count: ResMut<EnemyCount>
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Enemy,
            SpriteBundle { // dont ever query for the bundle type, aka the SpriteBundle
                sprite: Sprite { // instead of using a default, you can use a texture
                    color: Color::rgb(0.9, 0.1, 0.2),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(rng.gen_range(-300.0..300.0),rng.gen_range(-300.0..300.0), 0.),
                ..default()
            },
            Velocity( Vec2 { x: rng.gen_range(-100.0..-10.0) , y: 0.0 } )
        ));

        enemy_count.0 += 1;
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
        if transform.translation.x < -window.width() / 2.0 { 
            commands.entity(entity).despawn();
            enemy_count.0 -= 1;
        }
    }

}