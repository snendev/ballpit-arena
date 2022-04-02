use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod menu;
pub mod game;
pub mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
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
        .add_state(AppState::Menu)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_ui_camera)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}
