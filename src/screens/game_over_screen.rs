use bevy::prelude::*;
use crate::common::*;


#[derive(Component)]
struct GameOverMenu;
pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(listen_for_game_over)
        .add_systems((check_for_restart_or_quit, ).in_set(OnUpdate(AppState::GameOver)));
    }
}

fn listen_for_game_over (
    game_over_event_reader: EventReader<GameOverEvent>,
    commands: Commands, 
    window: Query<&Window>,
    asset_server: Res<AssetServer>
) { // this system will be used during all states
    if game_over_event_reader.len() >= 1 {
        spawn_game_over_screen(commands, window, asset_server);
    }
}

fn check_for_restart_or_quit ( // GameOver state only, listen for (A)/(X)/Space for restart, or (B)/Esc for exit game
    keys: Res<Input<KeyCode>>,
    mut restart_game_event: EventWriter<GameRestartEvent>,
    game_over_screen_entites: Query<Entity, With<GameOverMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamePad>>,
    
) {
    if keys.just_pressed(KeyCode::Space) {
        restart_game_event.send(GameRestartEvent);
        for entity in game_over_screen_entites.iter() {
            commands.entity(entity).despawn();
        }

        next_state.set(AppState::InGame);
        return;
    }

    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let restart_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::RightTrigger2
    };

    if buttons.just_pressed(restart_button) {
        restart_game_event.send(GameRestartEvent);
        for entity in game_over_screen_entites.iter() {
            commands.entity(entity).despawn();
        }

        next_state.set(AppState::InGame);
        return;
    }
}


fn spawn_game_over_screen (
    mut commands: Commands,
    window: Query<&Window>,
    asset_server: Res<AssetServer>
) {
    let (sprite_bundle, text_bundle) = create_screen(window, asset_server, "Game Over - RT/R2/Space - Restart");
    commands.spawn((
        GameOverMenu,
        sprite_bundle,
    ));

    commands.spawn((
        GameOverMenu,
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
            font_size: 25.0,
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