use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{enemy::Enemy, common::AppState};

#[derive(Resource)]
pub struct StageTimer(Timer);

#[derive(Resource)]
pub struct AllStages(VecDeque<Stage>);

#[derive(Resource)]
pub struct CurrentStage(pub Stage);

#[derive(Component)]
pub struct Stage {
    pub enemy_types: Vec<Enemy>,
    pub enemy_spawn_rate_sec: f32, 
    pub timer_limit_sec: f32

}

pub struct StageManagerPlugin;
impl Plugin for StageManagerPlugin { // TODO add this plugin to the main one
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .insert_resource(CurrentStage(
            Stage {
                enemy_types: Vec::from([Enemy::Standard]),
                enemy_spawn_rate_sec: 2.0,
                timer_limit_sec: 15.0
            },
        ))
        .insert_resource(StageTimer(Timer::from_seconds(15.0, TimerMode::Once)))
        .insert_resource(AllStages(
            VecDeque::from(
            [
                Stage {
                    enemy_types: Vec::from([Enemy::Standard]),
                    enemy_spawn_rate_sec: 1.0,
                    timer_limit_sec: 15.0
                },
                Stage {
                    enemy_types: Vec::from([Enemy::Standard]),
                    enemy_spawn_rate_sec: 0.5,
                    timer_limit_sec: 15.0
                }
            ]
        )))
        .add_systems(
            (stage_logic, ).in_set(OnUpdate(AppState::InGame))
        );
    }
}

fn stage_logic(
    time: Res<Time>,
    mut current_stage: ResMut<CurrentStage>,
    mut all_stages: ResMut<AllStages>,
    mut timer: ResMut<StageTimer>,
){
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some(stage) = all_stages.0.pop_front() {
        println!("NEXT LEVEL");
        timer.0 = Timer::from_seconds(current_stage.0.timer_limit_sec, TimerMode::Once);
        current_stage.0 = stage;
    }
}