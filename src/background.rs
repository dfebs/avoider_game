use bevy::prelude::*;
use rand::Rng;

use crate::common::Velocity;

#[derive(Component)]
pub struct Star;

pub fn star_movement(
    window: Query<&Window>,
    time: Res<Time>, 
    mut sprite: Query<(&mut Transform, &Velocity), With<Star>>
) {
    let window = window.single();

    let left_bound = -window.width() / 2.0; // TODO make a WindowBounds struct resource that will make these bounds reusable
    let right_bound = window.width() / 2.0;


    for (mut transform, vel) in &mut sprite {
        transform.translation.x += vel.0.x * time.delta_seconds();
        if transform.translation.x < left_bound - 8.0 {
            transform.translation.x = right_bound + 8.0;
        }
    }
}
pub fn spawn_stars(mut commands: Commands, window: Query<&Window>) {
    let mut rng = rand::thread_rng();
    let window = window.single();

    let left_bound = -window.width() / 2.0; // TODO make a WindowBounds struct resource that will make these bounds reusable
    let right_bound = window.width() / 2.0;
    let upper_bound = window.height() / 2.0;
    let lower_bound = -window.height() / 2.0;

    for _ in 1..=500 {
        commands.spawn((
            Star,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(4.0, 4.0)),
                    color: Color::rgb(155.0, 155.0, 155.0),
                    ..default()
                },
                transform: Transform::from_xyz(rng.gen_range(left_bound..right_bound), rng.gen_range(lower_bound..upper_bound), 0.5),
                ..default()
            },
            Velocity( Vec2 { x: -12.0, y: 0.0 } )
        ));
    }
}