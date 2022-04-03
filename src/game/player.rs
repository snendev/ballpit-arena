use bevy::prelude::*;
use bevy_prototype_lyon::{entity::Path, prelude::*};
use std::f32::consts::PI;

use crate::{
    AppState,
    audio,
    game::{
        level::{Durability, BRICK_APOTHEM, LEVEL_HEIGHT, LEVEL_WIDTH},
        scoreboard::Score,
    },
};

pub mod ai;
pub mod collision;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;

#[derive(PartialEq, Component)]
pub enum Activity {
	Idle,
    Flinch,
    Land(Direction),
    Jump,
    Jab,
    Stomp,
    Counter,
}
impl Default for Activity {
    fn default() -> Self {
        Activity::Idle
    }
}

#[derive(Component, Default)]
pub struct ActivityTimer(pub f32);
#[derive(Component, Default)]
pub struct Hype(i32, f32);
#[derive(Component, Default)]
pub struct Combo(i32, f32);
#[derive(Component, Default)]
pub struct Velocity(pub f32, pub f32);
#[derive(Component, Default)]
pub struct JumpCounter(pub u8);

#[derive(PartialEq)]
enum FacingDirection {
    Left,
    Right,
}

impl Default for FacingDirection {
	fn default() -> Self {
		FacingDirection::Right
	}
}

#[derive(Component, Default)]
pub struct Facing(FacingDirection);

// semantic nitpicking: this could be called "acceleration", but since it is
// declared by the divine heavens of the input manager, Input Influence seems
// slightly more appropriate 
#[derive(Component, Default)]
pub struct InputInfluence(pub f32, pub f32);

#[derive(Default, Bundle)]
pub struct CharacterBundle {
	activity: Activity,
    timer: ActivityTimer,
	hype: Hype,
	combo: Combo,
    velocity: Velocity,
    influence: InputInfluence,
    jumps: JumpCounter,
    facing: Facing,
}

pub type CharacterFilter = (With<Activity>, With<ActivityTimer>, With<Hype>, With<Combo>, With<Velocity>);

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const PLAYER_RADIUS: f32 = 25.;

pub const JUMP_SPEED: f32 = 200.;
pub const GRAVITY_Y: f32 = -200.;
const GRAVITY_Y_PER_STEP: f32 = GRAVITY_Y * TIME_STEP;

pub fn setup(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: PLAYER_RADIUS,
        ..Default::default()
    };

    commands.spawn()
        .insert(Player)
        .insert_bundle(CharacterBundle::default())
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 4.0),
            },
            Transform::default(),
        ));

    let mut transform2 = Transform::default();
    transform2.translation.x -= 100.;
    
    commands.spawn()
        .insert(Enemy)
        .insert(ai::Behavior::Chasing)
        .insert_bundle(CharacterBundle::default())
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE_RED),
                outline_mode: StrokeMode::new(Color::BLACK, 4.0),
            },
            transform2,
        ));

    let mut transform3 = Transform::default();
    transform3.translation.x += 100.; 
    commands.spawn()
        .insert(Enemy)
        .insert(ai::Behavior::Chasing)
        .insert_bundle(CharacterBundle::default())
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE_RED),
                outline_mode: StrokeMode::new(Color::BLACK, 4.0),
            },
            transform3,
        ));
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
    mut query: Query<
        (Entity, &mut Path, &Activity, &mut ActivityTimer, &mut JumpCounter, &mut Transform, &mut Velocity, &Facing),
        Changed<Activity>,
    >,
    mut audio_writer: EventWriter<audio::Event>,
) {
    // each query result is a shape that has just changed activity
    // update its shape to match the activity
    for (
        character,
        mut path,
        activity,
        mut timer,
        mut jumps,
        mut transform,
        mut velocity,
        facing,
    ) in query.iter_mut() {
        match activity {
            Activity::Idle => {
                let shape = shapes::Circle {
                    radius: PLAYER_RADIUS,
                    ..Default::default()
                };
                update_path(&mut path, &shape);
            }
            Activity::Flinch => {
                let shape = shapes::Circle {
                    radius: PLAYER_RADIUS - 2.,
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 1.;
            }
            Activity::Land(direction) => {
                let shape = if direction == &Direction::Top || direction == &Direction::Bottom {
                    shapes::Ellipse {
                        radii: Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS - 5.),
                        ..Default::default()
                    }
                } else {
                    shapes::Ellipse {
                        radii: Vec2::new(PLAYER_RADIUS - 5., PLAYER_RADIUS),
                        ..Default::default()
                    }
                };
                update_path(&mut path, &shape);
                timer.0 = 0.7;
            }
            Activity::Jump => {
                let shape = shapes::Ellipse {
                    radii: Vec2::new(PLAYER_RADIUS - 5., PLAYER_RADIUS),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                velocity.1 = JUMP_SPEED;
                jumps.0 -= 1;
                timer.0 = 0.5;
                audio_writer.send(audio::Event(character, audio::Trigger::CharacterJump, audio::Offset(0.)));
            }
            Activity::Jab => {
                let shape = shapes::RegularPolygon {
                    sides: 3,
                    feature: shapes::RegularPolygonFeature::Apothem(PLAYER_RADIUS - 5.),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                let sign = if facing.0 == FacingDirection::Left { 1. } else { -1. };
                transform.rotation = Quat::from_axis_angle(
                    Vec3::new(0., 0., 1.),
                    sign * PI / 2.
                );
                timer.0 = 0.9;
                audio_writer.send(audio::Event(character, audio::Trigger::CharacterAttack, audio::Offset(1.)));
            }
            Activity::Stomp => {
                let shape = shapes::RegularPolygon {
                    sides: 4,
                    feature: shapes::RegularPolygonFeature::Apothem(PLAYER_RADIUS),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 2.;
                audio_writer.send(audio::Event(character, audio::Trigger::CharacterAttack, audio::Offset(-2.)));
            }
            Activity::Counter => {
                let shape = shapes::RegularPolygon {
                    sides: 8,
                    feature: shapes::RegularPolygonFeature::Apothem(PLAYER_RADIUS),
                    ..Default::default()
                };
                update_path(&mut path, &shape);
                timer.0 = 2.;
                audio_writer.send(audio::Event(character, audio::Trigger::CharacterAttack, audio::Offset(3.)));
            }
        }
    }
}

// square + circle
fn check_for_player_and_brick_collision(
    player_location: &Vec3,
    brick_location: &Vec3,
) -> Option<Direction> {
    collision::cmp_circle_and_square(
        player_location,
        PLAYER_RADIUS,
        brick_location,
        BRICK_APOTHEM,
    )
}

pub fn handle_turning(
    mut query: Query<(&mut Facing, &Activity, &InputInfluence), Changed<Velocity>>, 
) {
    for (mut facing, activity, influence) in query.iter_mut() {
        if *activity == Activity::Idle {
            if facing.0 == FacingDirection::Right && influence.0 < 0. {
                facing.0 = FacingDirection::Left;
            }
            if facing.0 == FacingDirection::Left && influence.0 > 0. {
                facing.0 = FacingDirection::Right;
            }
        }
    }
}

pub fn handle_physics(
    mut characters_query: Query<
        (&mut Velocity, &mut Activity, &mut JumpCounter, &mut Transform, &InputInfluence),
        Or<(With<Player>, With<Enemy>)>,
    >,
    mut bricks_query: Query<
        (&mut Durability, &Transform),
        Or<(With<Durability>, Without<Player>, Without<Enemy>)>,
    >,
) {
    for (
        mut velocity,
        mut activity,
        mut jumps,
        mut transform,
        influence,
    ) in characters_query.iter_mut() {
        // adjust the influence of inputs based on the character's state
        let adjusted_influence = match activity.as_ref() {
            Activity::Idle => (influence.0, influence.1),
            Activity::Flinch => (0., 0.),
            Activity::Land(direction) => {
                if *direction == Direction::Top || *direction == Direction::Bottom {
                    (influence.0 / 2., 0.)
                } else {
                    (0., influence.1 / 2.)
                }
            }
            Activity::Jump => (influence.0, influence.1),
            Activity::Jab => (influence.0 / 2., influence.1 / 2.),
            Activity::Stomp => (influence.0 / 2., influence.1 / 2.),
            Activity::Counter => (influence.0 / 2., influence.1 / 2.),
        };
        // predict next location
        let vec2_velocity = Vec2::new(velocity.0, velocity.1);
        let damping = 0.000003 * vec2_velocity.length_squared();
        let mut adjusted_velocity = (
            velocity.0 + 3. * adjusted_influence.0 - damping * (if vec2_velocity.x.is_sign_positive() { -1. } else { 1. }),
            velocity.1 + adjusted_influence.1 + GRAVITY_Y_PER_STEP - damping * (if vec2_velocity.y.is_sign_positive() { -1. } else { 1. }),
        );
        let next_location = Vec3::new(
            transform.translation.x + adjusted_velocity.0 * TIME_STEP,
            transform.translation.y + adjusted_velocity.1 * TIME_STEP,
            0.
        );

        // check for collisions with walls at speed
        for (mut durability, transform) in bricks_query.iter_mut() {
            if let Some(collision) = check_for_player_and_brick_collision(
                &next_location,
                &transform.translation,
            ) {
                let impact = if collision == Direction::Right || collision == Direction::Left {
                    adjusted_velocity.0
                } else {
                    adjusted_velocity.1
                };
                if (
                    collision == Direction::Right && adjusted_velocity.0.is_sign_positive()
                ) || (
                    collision == Direction::Left && adjusted_velocity.0.is_sign_negative()
                ) {
                    adjusted_velocity.0 = 0.;
                } else if (
                    collision == Direction::Top && adjusted_velocity.1.is_sign_positive()
                ) || (
                    collision == Direction::Bottom && adjusted_velocity.1.is_sign_negative()
                ) {
                    adjusted_velocity.1 = 0.;
                }

                jumps.0 = 2;
                if impact > 100. {
                    *activity = Activity::Land(collision);
                    durability.0 -= impact;
                }
            }
        }

        transform.translation.x += adjusted_velocity.0 * TIME_STEP;
        transform.translation.y += adjusted_velocity.1 * TIME_STEP;
        velocity.0 = adjusted_velocity.0;
        velocity.1 = adjusted_velocity.1;
    }
}

#[derive(Default)]
struct AttackResult {
    launch: Vec2
}

impl AttackResult {
    fn new(direction: Direction) -> Self {
        match direction {
            Direction::Top => Self {
                launch: Vec2::new(0., 1.),
            },
            Direction::Right => Self {
                launch: Vec2::new(PI / 3., PI / 6.),
            },
            Direction::Bottom => Self {
                launch: Vec2::new(0., -1.),
            },
            Direction::Left => Self {
                launch: Vec2::new(-PI / 3., PI / 6.),
            },
        }
    }
}

fn calculate_attack_collision(
    activity: &Activity,
    attacker: &Transform,
    facing: &Facing,
    target: &Transform,
) -> Option<AttackResult> {
    match activity {
        Activity::Jab => {
            // attacker: triangle; target: circle
            let triangle_center = Vec2::new(attacker.translation.x, attacker.translation.y);
            let tip_vertex = triangle_center + Vec2::new(
                if facing.0 == FacingDirection::Right { PLAYER_RADIUS - 5. } else { PLAYER_RADIUS - 5. },
                0.,
            );
            let vertical_vertices = (
                triangle_center + Vec2::new(0., - PLAYER_RADIUS / 2.),
                triangle_center + Vec2::new(0., PLAYER_RADIUS / 2.),
            );
            let maybe_collision = collision::cmp_circle_and_triangle(
                &target.translation,
                PLAYER_RADIUS,
                if facing.0 == FacingDirection::Right {
                    (vertical_vertices.0, tip_vertex, vertical_vertices.1)
                } else {
                    (vertical_vertices.0, vertical_vertices.1, tip_vertex)
                },
            );
            if (attacker.translation - target.translation).length() < PLAYER_RADIUS {
                let direction = if facing.0 == FacingDirection::Right { Direction::Right } else { Direction::Left };
                Some(AttackResult::new(direction))
            } else {
                None
            }
        },
        Activity::Stomp => {
            // attacker: square; target: circle
            let maybe_collision = collision::cmp_circle_and_square(
                &target.translation,
                PLAYER_RADIUS,
                &attacker.translation,
                PLAYER_RADIUS,
            );
            if let Some(direction) = maybe_collision {
                Some(AttackResult::new(direction))
            } else {
                None
            }
        },
        Activity::Counter => {
            // attacker: circle; target: circle
            if collision::cmp_circles(
                attacker.translation,
                target.translation,
                PLAYER_RADIUS,
            ) {
                Some(AttackResult::default())
            } else {
                None
            }
        },
        _ => None
    }
}

const LAUNCH_COEF: f32 = 70.;

pub fn handle_attack_collision(
    mut query: Query<(Entity, &mut Activity, &mut Hype, &mut Combo, &mut Velocity, &mut Transform, &Facing)>,
    mut writer: EventWriter<audio::Event>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([
        (entity1, mut activity1, mut hype1, mut combo1, mut velocity1, mut transform1, facing1),
        (entity2, mut activity2, mut hype2, mut combo2, mut velocity2, mut transform2, facing2),
    ]) = combinations.fetch_next() {
        let one_hits_two = calculate_attack_collision(
            activity1.as_ref(),
            transform1.as_ref(),
            facing1,
            transform2.as_ref(),
        );
        let mut has_collided = false;

        if let Some(collision_result) = one_hits_two {
            has_collided = true;
            match activity2.as_ref() {
                Activity::Counter => {
                    combo1.0 += 1;
                    combo1.1 = 6.;
                    hype2.0 += 1;
                    hype2.1 = 4.;
                }
                Activity::Flinch => {}
                _ => {
                    let new_velocity = collision_result.launch * LAUNCH_COEF * (1. + hype1.0 as f32 + combo2.0 as f32);
                    velocity2.0 = new_velocity.x;
                    velocity2.1 = new_velocity.y; 
                    combo2.0 += 1;
                    combo2.1 = 6.;
                    hype1.0 += 1;
                    hype1.1 = 4.;
                    *activity2 = Activity::Flinch;
                }
            }
            writer.send(audio::Event(
                entity2,
                audio::Trigger::CharacterHit,
                audio::Offset(-3. + hype1.0 as f32 + combo2.0 as f32),
            ));
        }
        let two_hits_one = calculate_attack_collision(
            activity2.as_ref(),
            transform2.as_ref(),
            facing2,
            transform1.as_ref(),
        );
        if let Some(collision_result) = two_hits_one {
            has_collided = true;
            match activity1.as_ref() {
                Activity::Counter => {
                    combo2.0 += 1;
                    combo2.1 = 1.5;
                    hype1.0 += 1;
                }
                Activity::Flinch => {}
                _ => {
                    combo1.0 += 1;
                    combo1.1 = 1.5;
                    let new_velocity = collision_result.launch * LAUNCH_COEF * (1. + hype2.0 as f32) * (1. + combo1.0 as f32);
                    velocity1.0 = new_velocity.x;
                    velocity1.1 = new_velocity.y;
                    hype2.0 += 1;
                    *activity1 = Activity::Flinch;
                }
            }
            writer.send(audio::Event(
                entity1,
                audio::Trigger::CharacterHit,
                audio::Offset(-3. + hype2.0 as f32 + combo1.0 as f32),
            ));
        }
    }
}

pub fn handle_status_tick(
    mut query: Query<(&mut Hype, &mut Combo)>,
    time: Res<Time>,
) {
    for (mut hype, mut combo) in query.iter_mut() {
        let dt = time.delta().as_secs_f32();
        hype.1 -= dt;
        if hype.1 <= 0. {
            if hype.0 > 0 {
                hype.0 -= 1;
                hype.1 += 1.;
            } else {
                hype.1 = 0.;
            }
        }
        combo.1 -= dt;
        if combo.1 <= 0. {
            if combo.0 > 0 {
                combo.0 -= 1;
                combo.1 += 1.;
            } else {
                combo.1 = 0.;
            }
        }
    }
}

pub fn handle_status_change(
    mut query: Query<
        (&mut DrawMode, &Hype, &Combo, Option<&Player>, Option<&Enemy>),
        Or<(Changed<Hype>, Changed<Combo>)>,
    >,
) {
    for (mut draw_mode, hype, combo, player, enemy) in query.iter_mut() {
        let fill = match *draw_mode {
            DrawMode::Outlined { fill_mode, outline_mode } => { fill_mode.color }
            DrawMode::Fill(fill_mode) => { fill_mode.color }
            DrawMode::Stroke(stroke_mode) => { stroke_mode.color }
        };
        let outline = Color::rgb(0.1 + 0.3 * (combo.0 as f32), 0.1, 0.1 + 0.3 * (hype.0 as f32));
        *draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(fill),
            outline_mode: StrokeMode::new(outline, 4.0),
        }
    }
}

pub struct EnemySpawnTimer(f32);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(10.)
    }
}

pub fn handle_enemy_spawn_timer(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    query: Query<&Enemy>,
) {
    let num_enemies = query.iter().count();
    if num_enemies > 6 { return }

    spawn_timer.0 -= TIME_STEP;
    if spawn_timer.0 <= 0. {
        let shape = shapes::Circle {
            radius: PLAYER_RADIUS,
            ..Default::default()
        };
        commands.spawn()
            .insert(Enemy)
            .insert(ai::Behavior::Chasing)
            .insert_bundle(CharacterBundle::default())
            .insert_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::ORANGE_RED),
                    outline_mode: StrokeMode::new(Color::BLACK, 4.0),
                },
                Transform::default(),
            ));

        *spawn_timer = EnemySpawnTimer::default();
    }
}

fn is_offstage(translation: Vec3) -> bool {
    let is_offstage_right = translation.x > LEVEL_WIDTH;
    let is_offstage_left = translation.x < -LEVEL_WIDTH;
    let is_offstage_bottom = translation.y < -LEVEL_HEIGHT;
    let is_offstage_top = translation.y > LEVEL_HEIGHT;
    is_offstage_top || is_offstage_bottom || is_offstage_left || is_offstage_right
}

pub fn detect_enemy_death_system(
	mut commands: Commands,
	mut score: ResMut<Score>,
	mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let mut count = 0;
    for (enemy, transform) in enemy_query.iter_mut() {
        if is_offstage(transform.translation) {
            count += 1;
            commands.entity(enemy).despawn()
        }
	}
    score.0 +=  count;
}

pub fn detect_gameover_system(
    mut commands: Commands,
	mut state: ResMut<State<AppState>>,
	player_query: Query<(Entity, &mut Transform), With<Player>>,
) {
	for (entity, player) in player_query.iter() {
		if is_offstage(player.translation) {
            state.set(AppState::GameOver).unwrap();
        }
	}
}
