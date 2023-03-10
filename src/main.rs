use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Position(String);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_system(enemy_spawning)
        .add_system(enemy_movement);
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(player_movement)
        .add_system(handle_keyboard_input);
    }
}

fn player_movement(
    time: Res<Time>, 
    mut sprite: Query<(&mut Transform, &mut Velocity), 
    With<Player>>
) { // basically get the transform of all things that are sprites
    for (mut transform, mut vel) in &mut sprite {
        transform.translation.x += vel.0.x * time.delta_seconds();
        transform.translation.y += vel.0.y * time.delta_seconds();
        vel.0.x = 0.0;
        vel.0.y = 0.0;
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>, 
    mut sprite: Query<&mut Velocity, With<Player>>
) {
    let mut vel = sprite.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        println!("Hell ya bruther");
    }
    if keys.pressed(KeyCode::W) {
        vel.0.y = 100.0;
    }
    if keys.pressed(KeyCode::A) {
        vel.0.x = -100.0;
    }
    if keys.pressed(KeyCode::S) {
        vel.0.y = -100.0;
    }
    if keys.pressed(KeyCode::D) {
        vel.0.x = 100.0;
    }
}

fn enemy_spawning(
    mut commands: Commands, 
    time: Res<Time>, 
    mut timer: ResMut<EnemySpawnTimer>,
    // enemies: Query<Entity, With<Enemy>>
) {
    let mut rng = rand::thread_rng();
    // lets just spawn dudes 
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
            Velocity( Vec2 { x: 0.0 , y: 0.0 } )
        ));
    }
}
fn enemy_movement() {}

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
// Implement enemy movement
// Add controller support
// Move Enemy and Player related stuff to their own rust files