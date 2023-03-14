use bevy::prelude::Component;
use bevy::math::Vec2;
use bevy::ecs::schedule::States;
#[derive(Component)]
pub struct Position(String);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Projectile;

pub const PROJECTILE_HITBOX: Vec2 = Vec2::new(32.0, 16.0);

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    InGame,
    GameOver
}