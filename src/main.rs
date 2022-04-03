#![feature(drain_filter)]

use bevy::prelude::*;

pub mod audio;
pub mod game;
pub mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Game,
    GameOver,
}

fn setup_ui_camera(mut commands: Commands) {
    // HUD and menu screen
    commands.spawn_bundle(UiCameraBundle::default());
    // game rendering
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(audio::AudioPlugin)
        .add_state(AppState::Game)
        .add_startup_system(setup_ui_camera)
        .add_plugin(game::GamePlugin)
        .run();
}
