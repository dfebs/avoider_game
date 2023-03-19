use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
struct PauseMenu;

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(handle_keyboard_input);
    }
}

fn handle_keyboard_input (
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
    window: Query<&Window>,
    pause_menu_entities: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>
) {
    if keys.just_pressed(KeyCode::Escape) {
        let next_state = match app_state.0 {
            AppState::InGame => {
                spawn_pause_menu(commands, window, asset_server);
                AppState::Paused
            },
            AppState::Paused => {
                for entity in pause_menu_entities.iter() {
                    commands.entity(entity).despawn();
                }
                
                AppState::InGame
            },
            _ => AppState::GameOver
        };

        app_state.0 = next_state;
    }
}

fn spawn_pause_menu (
    mut commands: Commands,
    window: Query<&Window>,
    asset_server: Res<AssetServer>
) {
    let window = window.single();
    let width = window.width(); // TODO make a WindowBounds struct resource that will make these bounds reusable
    let height = window.height();
    commands.spawn((
        PauseMenu,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
            ..default()
    },
    ));

    commands.spawn((
        PauseMenu,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Paused",
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
        }),
    ));
}