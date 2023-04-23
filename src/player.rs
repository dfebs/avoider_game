use crate::common::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerWeaponCooldownTimer(Timer);

pub const PLAYER_HITBOX: Vec2 = Vec2::new(34.0, 54.0); // Player sprite is 64x64, this is more lenient
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(MyGamePad(Gamepad { id: 0 }))
        .add_startup_systems(
            (setup, ).in_set(OnUpdate(AppState::InGame))
        )
        .add_systems(
            (player_movement, handle_cooldowns, handle_keyboard_input, handle_gamepad_input, projectile_movement).in_set(OnUpdate(AppState::InGame))
        )
        .add_systems(
            (listen_for_game_restart, ).in_set(OnUpdate(AppState::GameOver))
        );
    }
}

fn setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(( // TODO move to player file
        Player,
        SpriteBundle {
            texture: asset_server.load("space_ship_player.png"),
            transform: Transform::from_xyz(100.,0., 1.0),
            ..default()
        },
        Velocity( Vec2 { x: 100.0 , y: 100.0 } )
    ));
}

fn listen_for_game_restart(
    game_restart_event_reader: EventReader<GameRestartEvent>,
    commands: Commands, 
    asset_server: Res<AssetServer>
) {
    if game_restart_event_reader.len() >= 1 {
        setup (commands, asset_server)
    }
}

fn fire_weapon (
    player_transform: &Transform,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut weapon_cooldown: Query<&mut PlayerWeaponCooldownTimer>,
    audio: Res<Audio>
) {
    match weapon_cooldown.get_single_mut() {
        Ok(timer) => {
            if !timer.0.finished() {
                return;
            } 
        }
        Err(_) => {
            commands.spawn(
                PlayerWeaponCooldownTimer(Timer::from_seconds(0.5, TimerMode::Once))
            );
        }
    }

    let pew_noise = asset_server.load("sound/laser.ogg");
    audio.play(pew_noise);
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

fn handle_cooldowns(
    mut commands: Commands,
    mut weapon_cooldown: Query<(Entity, &mut PlayerWeaponCooldownTimer)>,
    time: Res<Time>
) {
    if let Ok((entity, mut timer)) = weapon_cooldown.get_single_mut() {
        if !timer.0.tick(time.delta()).finished() {
            return;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_movement( // turn into a plugin at some point probably
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
    mut sprite: Query<(&mut Transform, &mut Velocity), With<Player>>,
    window: Query<&Window>
) {
    for (mut transform, mut vel) in &mut sprite {
        let window = window.single();
        transform.translation.x += vel.0.x * time.delta_seconds();
        transform.translation.y += vel.0.y * time.delta_seconds();

        if transform.translation.x > (window.width() / 2.0) - 32.0 { // There's definitely a better way of doing this
            transform.translation.x = (window.width() / 2.0) - 32.0;
        }

        if transform.translation.x < (-window.width() / 2.0) + 32.0 { 
            transform.translation.x = (-window.width() / 2.0) + 32.0;
        }

        if transform.translation.y > (window.height() / 2.0) - 32.0 { 
            transform.translation.y = (window.height() / 2.0) - 32.0;
        }

        if transform.translation.y < (-window.height() / 2.0) + 32.0 { 
            transform.translation.y = (-window.height() / 2.0) + 32.0;
        }
        vel.0.x = 0.0;
        vel.0.y = 0.0;
    }
}

fn handle_keyboard_input(
    commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Transform, &mut Velocity), With<Player>>,
    weapon_cooldown: Query<&mut PlayerWeaponCooldownTimer>,
    audio: Res<Audio>
) {
    let (transform, mut vel) = player.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        fire_weapon(transform, commands, asset_server, weapon_cooldown, audio);
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

fn handle_gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamePad>>,

    commands: Commands,
    asset_server: Res<AssetServer>, // Not ideal, we'd ideally have assets preloaded and not have to do this
    audio: Res<Audio>,
    weapon_cooldown: Query<&mut PlayerWeaponCooldownTimer>,
    mut player: Query<(&Transform, &mut Velocity), With<Player>>
){
    let (transform, mut vel) = player.single_mut();
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx),  axes.get(axis_ly)) {
        let left_stick_pos = Vec2::new(x, y);
        vel.0.x = 200.0 * left_stick_pos.x;
        vel.0.y = 200.0 * left_stick_pos.y;
    } 

    let shoot_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::RightTrigger2
    };

    if buttons.just_pressed(shoot_button) {
        fire_weapon(transform, commands, asset_server, weapon_cooldown, audio);
    }

}