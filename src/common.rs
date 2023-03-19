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

#[derive(Component)]
pub struct AnimationTimer(Timer);

#[derive(Resource)]
pub struct ExplosionSprite(pub Handle<TextureAtlas>);

pub const PROJECTILE_HITBOX: Vec2 = Vec2::new(32.0, 16.0);

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_systems(
            (detect_collisions, animate_explosions).in_set(OnUpdate(AppState::InGame))
        );
    }
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

fn animate_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite
    )>
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if sprite.index == 11 {
                commands.entity(entity).despawn();
            }
            sprite.index += 1;
        }
    }
}

fn detect_collisions(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>, 
    player_projectiles: Query<(Entity, &Transform), With<Projectile>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut app_state: ResMut<State<AppState>>,
    mut enemy_count: ResMut<EnemyCount>,
    explosion_sprite: Res<ExplosionSprite>
) {
    let player_transform = player.single();

    for (entity, enemy_transform) in enemies.iter() {
        if let Some(_) = collide(player_transform.translation, PLAYER_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
            app_state.0 = AppState::GameOver;
        }

        for (projectile, projectile_transform) in player_projectiles.iter() {
            if let Some(_) = collide(projectile_transform.translation, PROJECTILE_HITBOX, enemy_transform.translation, ENEMY_HITBOX) {
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: explosion_sprite.0.clone(),
                        sprite: TextureAtlasSprite::new(0),
                        transform: Transform::from_xyz(enemy_transform.translation.x, enemy_transform.translation.y, 5.0),
                        // transform: Transform::from_scale(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 5.0)), // this will need to change
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                ));
                commands.entity(entity).despawn();
                commands.entity(projectile).despawn();
                enemy_count.0 -= 1;
            }
        }
    }
}