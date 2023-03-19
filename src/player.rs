use crate::common::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub const PLAYER_HITBOX: Vec2 = Vec2::new(34.0, 54.0); // Player sprite is 64x64, this is more lenient
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            (player_movement, handle_keyboard_input, projectile_movement).in_set(OnUpdate(AppState::InGame))
        );
    }
}

fn fire_weapon (
    player_transform: &Transform,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Projectile,
        SpriteBundle {
            texture: asset_server.load("projectile.png"),
            transform: Transform::from_xyz(player_transform.translation.x + 38.0 ,player_transform.translation.y, 1.0),
            ..default()
        },
        Velocity( Vec2 { x: 700.0 , y: 0.0 } )
    ));
}

pub fn projectile_movement( // turn into a plugin at some point probably
    time: Res<Time>, 
    mut sprite: Query<(&mut Transform, &Velocity), With<Projectile>>
) {
    for (mut transform, vel) in &mut sprite {
        transform.translation.x += vel.0.x * time.delta_seconds();
        transform.translation.y += vel.0.y * time.delta_seconds();
    }
}

fn player_movement(
    time: Res<Time>, 
    mut sprite: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    for (mut transform, mut vel) in &mut sprite {
        transform.translation.x += vel.0.x * time.delta_seconds();
        transform.translation.y += vel.0.y * time.delta_seconds();
        vel.0.x = 0.0;
        vel.0.y = 0.0;
    }
}

fn handle_keyboard_input(
    commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>, 
    mut app_state: ResMut<State<AppState>>,
    mut sprite: Query<(&Transform, &mut Velocity), With<Player>>
) {
    let (transform, mut vel) = sprite.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        fire_weapon(transform, commands, asset_server);
    }
    if keys.pressed(KeyCode::W) {
        vel.0.y = 200.0;
    }
    if keys.pressed(KeyCode::A) {
        vel.0.x = -200.0;
    }
    if keys.pressed(KeyCode::S) {
        vel.0.y = -200.0;
    }
    if keys.pressed(KeyCode::D) {
        vel.0.x = 200.0;
    }
}
