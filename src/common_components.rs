use bevy::prelude::Component;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Position(String);

#[derive(Component)]
pub struct Velocity(pub Vec2);