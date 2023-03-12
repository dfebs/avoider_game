use bevy::prelude::Component;
use bevy::math::Vec2;
use bevy::ecs::schedule::States;

#[derive(Component)]
pub struct Position(String);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    InGame,
    GameOver
}