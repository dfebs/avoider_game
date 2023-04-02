use bevy::prelude::*;

use self::pause_screen::PausePlugin;
use self::game_over_screen::GameOverPlugin;
use self::stats_overlay_screen::*;

mod pause_screen;
mod game_over_screen;
mod stats_overlay_screen;

pub struct ScreenManagerPlugin;
impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PausePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(StatsOverlayPlugin);
    }
}