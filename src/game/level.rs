use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BRICK_APOTHEM: f32 = 15.;

const BRICKS_WIDE: i32 = 40;
const BRICKS_WIDE_F: f32 = 40.;
const BRICKS_TALL: i32 = 22;
const BRICKS_TALL_F: f32 = 22.;

pub const LEVEL_WIDTH: f32 = BRICKS_WIDE_F * BRICK_APOTHEM;
pub const LEVEL_HEIGHT: f32 = BRICKS_TALL_F * BRICK_APOTHEM;

#[derive(Component)]
pub struct Durability(f32);

impl Default for Durability {
	fn default() -> Self {
		Durability(10.)
	}
}

type BrickFilter = With<Durability>;

pub fn setup(mut commands: Commands) {
    let brick_shape = shapes::RegularPolygon {
		sides: 4,
		feature: shapes::RegularPolygonFeature::Apothem(BRICK_APOTHEM),
		..Default::default()
	};

	let mut spawn_brick = |x: f32, y: f32| {
		commands.spawn()
			.insert(Durability::default())
			.insert_bundle(GeometryBuilder::build_as(
				&brick_shape,
				DrawMode::Outlined {
					fill_mode: FillMode::color(Color::BLACK),
					outline_mode: StrokeMode::new(Color::MAROON, 4.0),
				},
				Transform::from_xyz(x, y, 0.),
			));
	};

	// spawn top wall
	for count in 0..=BRICKS_WIDE {
		let position: f32 = (count - BRICKS_WIDE / 2) as f32;
		let x: f32 = BRICK_APOTHEM * 2. * position;
		spawn_brick(x, BRICK_APOTHEM * BRICKS_TALL_F);
		spawn_brick(x, -BRICK_APOTHEM * BRICKS_TALL_F);
	}
	// spawn left wall
	for count in 1..BRICKS_TALL {
		let position: f32 = (count - BRICKS_TALL / 2) as f32;
		let y: f32 = BRICK_APOTHEM * 2. * position;
		spawn_brick(-BRICK_APOTHEM * BRICKS_WIDE_F, y);
		spawn_brick(BRICK_APOTHEM * BRICKS_WIDE_F, y);
	}
}

pub fn detect_brick_break(mut commands: Commands, query: Query<(Entity, &Durability), BrickFilter>) {
  for (brick, durability) in query.iter() {
    if durability.0 <= 0. {
      commands.entity(brick).despawn_recursive();
    }
  }
}
