use crate::common_components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub const PLAYER_SIZE: f32 = 50.0;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            (player_movement, handle_keyboard_input).in_set(OnUpdate(AppState::InGame))
        );
    }
}

fn player_movement(
    time: Res<Time>, 
    mut sprite: Query<(&mut Transform, &mut Velocity), 
    With<Player>>
) {
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
