use bevy::prelude::*;

use crate::common::{AppState, MyGamePad};

#[derive(Component)]
struct PauseMenu;

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((handle_keyboard_input, handle_gamepad_input));
    }
}

fn handle_keyboard_input (
    keys: Res<Input<KeyCode>>,
    app_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    window: Query<&Window>,
    pause_screen_entities: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let next_state_to_go = match app_state.0 {
            AppState::InGame => {
                spawn_pause_screen(commands, window, asset_server);
                AppState::Paused
            },
            AppState::Paused => {
                for entity in pause_screen_entities.iter() {
                    commands.entity(entity).despawn();
                }
                
                AppState::InGame
            },
            _ => AppState::GameOver
        };

       next_state.set(next_state_to_go);
    }
}

fn handle_gamepad_input ( // This logic could potentially be merged with the keyboard input function
    app_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    window: Query<&Window>,
    pause_screen_entities: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamePad>>,
) {

    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let pause_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::Start
    };

    if buttons.just_pressed(pause_button)  {
        let next_state_to_go = match app_state.0 {
            AppState::InGame => {
                spawn_pause_screen(commands, window, asset_server);
                AppState::Paused
            },
            AppState::Paused => {
                for entity in pause_screen_entities.iter() {
                    commands.entity(entity).despawn();
                }
                
                AppState::InGame
            },
            _ => return
        };

       next_state.set(next_state_to_go);
    }
}

fn spawn_pause_screen (
    mut commands: Commands,
    window: Query<&Window>,
    asset_server: Res<AssetServer>
) {
    let (sprite_bundle, text_bundle) = create_screen(window, asset_server, "Paused");
    commands.spawn((
        PauseMenu,
        sprite_bundle,
    ));

    commands.spawn((
        PauseMenu,
        text_bundle
    ));
}

// I tried having this be a shared function that ONLY pause_screen, game_over_screen, and stats_overlay_screen could use.
// I could not for the life of me figure out how to do it. It's definitely because I don't know enough about the crate system.
// I could keep trying to figure it out but if I do, I think it's going to kill my drive enough for me to stop working on this, so here we are.
fn create_screen(
    window: Query<&Window>,
    asset_server: Res<AssetServer>,
    screen_text: &str
) -> (SpriteBundle, TextBundle) {
    let window = window.single();
    let width = window.width(); // TODO make a WindowBounds struct resource that will make these bounds reusable
    let height = window.height();

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ..default()
    };

    let text_bundle = TextBundle::from_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        screen_text,
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 100.0,
            color: Color::WHITE,
        },
    ) // Set the alignment of the Text
    .with_text_alignment(TextAlignment::Center)
    // Set the style of the TextBundle itself.
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect { // TODO-maybe, this is whack; there must be a better way of doing this
            bottom: Val::Px(height / 2.0),
            right: Val::Px(width / 3.0),
            ..default()
        },
        ..default()
    });

    return (sprite_bundle,text_bundle);
}