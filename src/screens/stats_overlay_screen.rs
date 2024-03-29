use bevy::prelude::*;
use crate::{common::AppState, stage_manager::CurrentStage};
// This file will function like the pause screen and the game over screen, but will instead serve as an overlay while the game is playing.
// It will only function ingame, and it will show enemies killed as what level the player is on.

#[derive(Component)]
pub struct StatsOverlayScreen;

pub struct StatsOverlayPlugin;
impl Plugin for StatsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(initialize)
        .add_system(maintain_screen_state.in_set(OnUpdate(AppState::InGame)))
        .add_system(remove_screen.in_schedule(OnEnter(AppState::GameOver)));
    }
}

fn initialize (
    commands: Commands,
    asset_server: Res<AssetServer>
) {
    spawn_screen(commands, asset_server, String::from("Level 1"));
}

fn spawn_screen (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    title: String
) {
    let text_bundle = create_screen(asset_server, title);

    commands.spawn((
        StatsOverlayScreen,
        text_bundle
    ));
}

fn maintain_screen_state (
    mut commands: Commands,
    current_stage: Res<CurrentStage>,
    text: Query<(Entity, &mut Text), With<StatsOverlayScreen>>,
    asset_server: Res<AssetServer>
) {
    if current_stage.0.is_none() {
        return;
    }

    if text.iter().len() == 0 {
        spawn_screen(commands, asset_server, String::from(&current_stage.0.as_ref().unwrap().title));
        return;
    }
    let (entity, title) = text.single();

    if current_stage.0.as_ref().unwrap().title == title.sections[0].value {
        return;
    }

    commands.entity(entity).despawn();

    spawn_screen(commands, asset_server, String::from(&current_stage.0.as_ref().unwrap().title))
}

fn remove_screen(
    mut commands: Commands,
    text: Query<Entity, With<StatsOverlayScreen>>,
) {
    let entity = text.single();
    commands.entity(entity).despawn();
}

fn create_screen(
    asset_server: Res<AssetServer>,
    screen_text: String
) -> TextBundle {
    let text_bundle = TextBundle::from_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        screen_text,
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 50.0,
            color: Color::WHITE,
        },
    ) // Set the alignment of the Text
    .with_text_alignment(TextAlignment::Center)
    // Set the style of the TextBundle itself.
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect { // TODO-maybe, this is whack; there must be a better way of doing this
            bottom: Val::Px(15.),
            right: Val::Px(15.),
            ..default()
        },
        ..default()
    });

    return text_bundle;
}