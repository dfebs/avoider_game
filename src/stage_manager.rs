use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{enemy::Enemy, common::{AppState, GameRestartEvent}};

#[derive(Resource)]
pub struct StageTimer(Timer);

#[derive(Resource)]
pub struct AllStages(VecDeque<Stage>);

#[derive(Resource)]
pub struct CurrentStage(pub Stage);

pub struct Stage {
    pub title: String,
    pub enemy_types: Vec<Enemy>,
    pub enemy_spawn_rate_sec: f32, 
    pub timer_limit_sec: f32
}

pub struct StageManagerPlugin;
impl Plugin for StageManagerPlugin {
    fn build(&self, app: &mut App) {
        let (starting_stage, starting_timer, remaining_stages) = generate_stages();

        app
        .insert_resource(CurrentStage(
            starting_stage
        ))
        .insert_resource(starting_timer)
        .insert_resource(AllStages(
            remaining_stages
        ))
        .add_systems(
            (stage_logic, ).in_set(OnUpdate(AppState::InGame))
        )
        .add_systems(
            (listen_for_game_restart, ).in_set(OnUpdate(AppState::GameOver))
        );
    }
}

fn listen_for_game_restart(
    game_restart_event_reader: EventReader<GameRestartEvent>,
    mut current_stage: ResMut<CurrentStage>,
    mut timer: ResMut<StageTimer>,
    mut all_stages: ResMut<AllStages>,
) {
    if game_restart_event_reader.len() >= 1 {
        (current_stage.0, *timer, all_stages.0) = generate_stages();
    }
    
}

fn stage_logic(
    time: Res<Time>,
    mut current_stage: ResMut<CurrentStage>,
    mut timer: ResMut<StageTimer>,
    mut all_stages: ResMut<AllStages>,
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

fn generate_stages () -> (Stage, StageTimer, VecDeque<Stage>) {
    const STARTING_TIME_LIMIT: f32 = 20.0;

    let first_stage = Stage {
        title: String::from("Level 1"),
        enemy_types: Vec::from([Enemy::Standard]),
        enemy_spawn_rate_sec: 2.0,
        timer_limit_sec: STARTING_TIME_LIMIT
    };

    let starting_timer = StageTimer(Timer::from_seconds(STARTING_TIME_LIMIT, TimerMode::Once));

    let remaining_stages =  VecDeque::from(
        [
            Stage {
                title: String::from("Level 2"),
                enemy_types: Vec::from([Enemy::Standard, Enemy::Wavy(1.0)]),
                enemy_spawn_rate_sec: 1.0,
                timer_limit_sec: 20.0
            },
            Stage {
                title: String::from("Level 3"),
                enemy_types: Vec::from([Enemy::Wavy(1.0), Enemy::Wavy(2.0)]),
                enemy_spawn_rate_sec: 0.5,
                timer_limit_sec: 20.0
            }
        ]
    );

    return (first_stage, starting_timer, remaining_stages);
}