use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{AppState, utils::destroy_recursive};

mod gameover;
pub mod input;
pub mod level;
pub mod player;
pub mod scoreboard;

pub fn handle_input_events(
    mut events: EventReader<input::Event>,
    mut action_query: Query<
        (&mut player::InputInfluence, &mut player::Activity, &player::JumpCounter),
        With<player::Player>,
    >,
    mut state: ResMut<State<AppState>>,
) {
    if let Some((
        mut influence,
        mut activity,
        jumps,
    )) = action_query.get_single_mut().ok() {
        for event in events.iter() {
            match &event.0 {
                input::Trigger::PlayerAction(action) => {
                    let can_execute_action = *activity == player::Activity::Idle || *activity == player::Activity::Counter;
                    let can_jump = jumps.0 > 0 && match activity.as_ref() {
                        player::Activity::Jump | player::Activity::Flinch | player::Activity::Land(_) => false,
                        _ => true,
                    };
                    match action {
                        input::ActionType::Jump => {
                            if can_jump {
                                *activity = player::Activity::Jump;
                            }
                        },
                        input::ActionType::Jab => {
                            if can_execute_action {
                                *activity = player::Activity::Jab;
                            }
                        },
                        input::ActionType::Stomp => {
                            if can_execute_action {
                                *activity = player::Activity::Stomp;
                            }
                        },
                        input::ActionType::Counter => {
                            if can_execute_action {
                                *activity = player::Activity::Counter;
                            }
                        },
                        _ => {},
                    }
                },
                input::Trigger::PlayerMovement(x, y) => {
                    influence.0 = *x;
                    influence.1 = *y;
                },
                input::Trigger::Pause => {
                    // state.set(AppState::Menu).unwrap();
                },
            }
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
            .add_plugin(ShapePlugin)
			.insert_resource(scoreboard::Score::default())
			.insert_resource(player::EnemySpawnTimer::default())
			.add_event::<input::Event>()
			.add_system_set(
				SystemSet::on_enter(AppState::Game)
					.with_system(level::setup)
					.with_system(player::setup)
                    .with_system(scoreboard::setup)
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game)
					.with_run_criteria(FixedTimestep::step(player::TIME_STEP as f64))
					.with_system(input::handle_gamepad_input.label("handle_gamepad_input"))
					.with_system(input::handle_keyboard_input.label("handle_keyboard_input").after("handle_gamepad_input"))
                    .with_system(player::ai::handle_ai_behavior)
                    .with_system(player::ai::handle_ai_input.label("handle_ai_input"))
                    .with_system(
                        handle_input_events
                            .label("handle_input_events")
                            .after("handle_gamepad_input")
                            .after("handle_keyboard_input")
                            .after("handle_ai_input")
                    )
					.with_system(
                        player::handle_activity_change
                            .label("handle_activity_change")
                            .after("handle_input_events")
                    )
					.with_system(
                        player::handle_activity_timer
                            .label("handle_activity_timer")
                            .after("handle_activity_change")) // TODO: make sure to add ordering dependencies
                    .with_system(
                        player::handle_physics
                            .label("physics")
                            .after("handle_activity_change")
                    )
                    .with_system(player::handle_attack_collision.after("physics"))
                    .with_system(player::handle_turning.after("physics"))
					.with_system(player::handle_enemy_spawn_timer)
                    .with_system(level::handle_brick_damage.after("physics"))
                    .with_system(level::handle_brick_break.after("physics"))
					.with_system(player::detect_enemy_death_system.after("physics"))
                    .with_system(player::handle_status_tick.label("handle_status_tick").after("physics"))
                    .with_system(player::handle_status_change.after("handle_status_tick").before("detect_gameover_system"))
                    .with_system(scoreboard::handle_tracking_score.before("detect_gameover_system"))
					.with_system(player::detect_gameover_system.label("detect_gameover_system").after("physics"))
			)
			.add_system_set(
				SystemSet::on_exit(AppState::Game)
					.with_system(destroy_recursive::<scoreboard::ScoreboardRootNode>)
                    .with_system(destroy_recursive::<player::Player>)
					.with_system(destroy_recursive::<player::Enemy>)
					.with_system(destroy_recursive::<level::Durability>)
			)
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver)
                    .with_system(gameover::setup)
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(gameover::handle_ui)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::GameOver)
                    .with_system(destroy_recursive::<gameover::GameOverRootNode>)
            );
	}
}
