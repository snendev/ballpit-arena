use bevy::{core::FixedTimestep, prelude::*};

use crate::{AppState, utils::destroy_recursive};

pub mod animation;
pub mod audio;
pub mod input;
pub mod player;
pub mod level;

// TODO HUD
#[derive(Component)]
struct GameUIRootNode;

pub fn handle_input_event(
    mut events: EventReader<input::InputEvent>,
    mut action_query: Query<(&mut player::Activity, &player::ActivityTimer), With<player::Player>>,
    mut state: ResMut<State<AppState>>,
) {
    if let Some((mut activity, timer)) = action_query.get_single_mut().ok() {
        for event in events.iter() {
            info!("{:?}", event);
            match &event.0 {
                input::InputEventType::PlayerAction(action) => {
                    if activity.0 == player::ActivityType::Idle && timer.0 <= 0. {
                        match action {
                            input::ActionType::Jump => {
                            },
                            input::ActionType::Jab => {
                                activity.0 = player::ActivityType::Jab;
                            },
                            input::ActionType::Spike => {
                                activity.0 = player::ActivityType::Spike;
                            },
                            input::ActionType::Counter => {
                                activity.0 = player::ActivityType::Counter;
                            },
                        }
                    }
                },
                input::InputEventType::Pause => {
                    state.set(AppState::Menu).unwrap();
                },
            }
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(player::Score::default())
			.insert_resource(player::EnemySpawnTimer::default())
			.add_event::<audio::AudioEvent>()
			.add_event::<input::InputEvent>()
			.add_system_set(
				SystemSet::on_enter(AppState::Game)
					.with_system(level::setup)
					.with_system(player::setup)
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game)
					.with_run_criteria(FixedTimestep::step(player::TIME_STEP as f64))
					.with_system(input::handle_input)
                    .with_system(handle_input_event)
					.with_system(player::handle_activity_change)
					.with_system(player::handle_activity_timer)
					.with_system(player::handle_enemy_spawn_timer)
					.with_system(player::detect_enemy_death_system)
					.with_system(player::detect_gameover_system)
			)
			.add_system_set(
				SystemSet::on_exit(AppState::Game)
					// TODO keep in mind this node isn't implemented yet
					.with_system(destroy_recursive::<GameUIRootNode>)
			);
	}
}
