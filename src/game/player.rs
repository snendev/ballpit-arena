use bevy::prelude::*;
use bevy_prototype_lyon::{entity::Path, prelude::*};

use crate::{
    AppState,
    game::level::{Durability, LEVEL_HEIGHT, LEVEL_WIDTH},
};

#[derive(Default)]
pub struct Score(i32);

#[derive(PartialEq)]
pub enum ActivityType {
	Idle,
    Flinch,
    Jab,
    Spike,
    Counter,
}

impl Default for ActivityType {
	fn default() -> Self {
		ActivityType::Idle
	}
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Activity(pub ActivityType);
#[derive(Component, Default)]
pub struct ActivityTimer(pub f32);
#[derive(Component, Default)]
pub struct Hype(i32);
#[derive(Component, Default)]
pub struct Combo(i32);
#[derive(Component, Default)]
pub struct Velocity(f32, f32);

#[derive(Default, Bundle)]
pub struct CharacterBundle {
	activity: Activity,
    timer: ActivityTimer,
	hype: Hype,
	combo: Combo,
    velocity: Velocity,
}

pub type CharacterFilter = (With<Activity>, With<ActivityTimer>, With<Hype>, With<Combo>, With<Velocity>);

pub const TIME_STEP: f32 = 1.0 / 60.0;

fn spawn_character<T: Component>(
    mut commands: Commands,
    transform: Transform,
    fill: Color,
    marker: T,
) {
    let shape = shapes::Circle {
        radius: 25.,
        ..Default::default()
    };
    commands.spawn()
        .insert(marker)
        .insert_bundle(CharacterBundle::default())
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(fill),
                outline_mode: StrokeMode::new(Color::MIDNIGHT_BLUE, 4.0),
            },
            transform,
        ));
}

pub fn setup(mut commands: Commands) {
    spawn_character(
        commands,
        Transform::from_xyz(-30., -30., 0.),
        Color::CYAN,
        Player
    );
}

pub fn handle_activity_timer(
    mut query: Query<(&mut Activity, &mut ActivityTimer)>,
) {
    for (mut activity, mut activity_timer) in query.iter_mut() {
        if activity_timer.0 > 0. {
            activity_timer.0 -= TIME_STEP;
        } else {
            *activity = Activity::default();
        }
    }
}

fn update_path<T: Geometry>(path: &mut Path, shape: &T) {
    let new_path = GeometryBuilder::build_as(
        shape,
        DrawMode::Fill(FillMode::color(Color::GRAY)),
        Transform::default(),
    ).path;
    *path = new_path;
}

pub fn handle_activity_change(
    mut query: Query<(&mut Path, &Activity, &mut ActivityTimer), Changed<Activity>>,
) {
    // each query result is a shape that has just changed activity
    // update its shape to match the activity
    for (mut path, activity, mut timer) in query.iter_mut() {
        match activity.0 {
            ActivityType::Idle => {
                let shape = shapes::Circle {
                    radius: 25.,
                    ..Default::default()
                };
                update_path(&mut path, &shape);
            }
            ActivityType::Flinch => {
                let shape = shapes::Circle {
                    radius: 23.,
                    ..Default::default()
                };
                update_path(&mut path, &shape);
            }
            ActivityType::Jab => {
                let shape = shapes::RegularPolygon {
                    sides: 3,
                    feature: shapes::RegularPolygonFeature::Apothem(25.),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 2.;
            }
            ActivityType::Spike => {
                let shape = shapes::RegularPolygon {
                    sides: 4,
                    feature: shapes::RegularPolygonFeature::Apothem(25.),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 2.;
            }
            ActivityType::Counter => {
                let shape = shapes::RegularPolygon {
                    sides: 8,
                    feature: shapes::RegularPolygonFeature::Apothem(25.),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 2.;
            }
        }
    }
}

pub fn handle_physics(
    mut characters_query: Query<(&mut Velocity, &mut Transform)>,
    mut walls_query: Query<(Entity, &Durability, &Transform)>,
) {

}


pub struct EnemySpawnTimer(f32);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(10.)
    }
}

pub fn handle_enemy_spawn_timer(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>
) {
    spawn_timer.0 -= TIME_STEP;
    if spawn_timer.0 <= 0. {
        spawn_character(
            commands,
            Transform::default(),
            Color::ORANGE_RED,
            Enemy,
        );
    }
}

pub fn detect_enemy_death_system(
	// mut commands: Commands,
	mut score: ResMut<Score>,
	mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
	let count: i32 = enemy_query.iter().count().try_into().unwrap();
	score.0 +=  count;
	for (enemy, transform) in enemy_query.iter_mut() {
		// commands.entity(enemy).despawn()
	}
}

pub fn detect_gameover_system(
	mut state: ResMut<State<AppState>>,
	player_query: Query<&mut Transform, With<Player>>,
) {
	for player in player_query.iter() {
		// if player
		// state.set(AppState::GameOver)
	}
}
