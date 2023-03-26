use bevy::{prelude::*, sprite::collide_aabb::collide};
use common::{Velocity, CommonPlugin, ExplosionSprite};
use enemy::EnemyPlugin;
use player::{Player, PlayerPlugin};
use stage_manager::*;
use background::*;
use screens::*;

mod common;
mod enemy;
mod player;
mod background;
mod stage_manager;
mod screens;

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, // TODO load all textures here
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("explosion_animation.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(192.0, 192.0), 4, 3, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    commands.insert_resource(ExplosionSprite(explosion));

}

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundPlugin)
        .add_plugin(CommonPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScreenManagerPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StageManagerPlugin)
        .run();
}