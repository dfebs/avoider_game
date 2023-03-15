use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::enemy::*;
use crate::player::*;
use bevy::math::Vec2;
use bevy::ecs::schedule::States;

#[derive(Component)]
pub struct Position(String);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Projectile;

pub const PROJECTILE_HITBOX: Vec2 = Vec2::new(32.0, 16.0);

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_systems(
            (detect_collisions,).in_set(OnUpdate(AppState::InGame))
        );
    }
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    InGame,
    GameOver
}

fn detect_collisions(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>, 
    player_projectiles: Query<(Entity, &Transform), With<Projectile>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut app_state: ResMut<State<AppState>>,
    mut enemy_count: ResMut<EnemyCount>
) {
    let player_transform = player.single();

    for (entity, enemy_transform) in enemies.iter() {
        if let Some(_) = collide(player_transform.translation, PLAYER_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
            app_state.0 = AppState::GameOver;
        }

        for (projectile, projectile_transform) in player_projectiles.iter() {
            if let Some(_) = collide(projectile_transform.translation, PROJECTILE_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
                commands.entity(entity).despawn();
                commands.entity(projectile).despawn();
                enemy_count.0 -= 1;
            }
        }
    }
}