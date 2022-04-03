use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::audio;

pub const BRICK_APOTHEM: f32 = 15.;

const BRICKS_WIDE: i32 = 40;
const BRICKS_WIDE_F: f32 = 40.;
const BRICKS_TALL: i32 = 22;
const BRICKS_TALL_F: f32 = 22.;

pub const LEVEL_WIDTH: f32 = BRICKS_WIDE_F * BRICK_APOTHEM;
pub const LEVEL_HEIGHT: f32 = BRICKS_TALL_F * BRICK_APOTHEM;

#[derive(Component)]
pub struct Durability(pub f32);

impl Default for Durability {
	fn default() -> Self {
		Durability(1000.)
	}
}

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

	// spawn top and bottom wall
	for count in 0..=BRICKS_WIDE {
		let position: f32 = (count - BRICKS_WIDE / 2) as f32;
		let x: f32 = BRICK_APOTHEM * 2. * position;
		spawn_brick(x, BRICK_APOTHEM * BRICKS_TALL_F);
		spawn_brick(x, -BRICK_APOTHEM * BRICKS_TALL_F);
	}
	// spawn left and right wall
	for count in 1..BRICKS_TALL {
		let position: f32 = (count - BRICKS_TALL / 2) as f32;
		let y: f32 = BRICK_APOTHEM * 2. * position;
		spawn_brick(-BRICK_APOTHEM * BRICKS_WIDE_F, y);
		spawn_brick(BRICK_APOTHEM * BRICKS_WIDE_F, y);
	}
}

pub fn handle_brick_damage(
	mut query: Query<(&Durability, &mut DrawMode), Changed<Durability>>,
) {
	for (durability, mut draw_mode) in query.iter_mut() {
        let durability = durability.0;
        let maybe_colors = if 0. < durability && durability <= 200. {
            Some((
                Color::rgb(0.2, 0.2, 0.2),
                Color::rgb(0.27, 0.0, 0.0),
            ))
        } else if 200. < durability && durability <= 400. {
            Some((
                Color::rgb(0.14, 0.1, 0.14),
                Color::rgb(0.33, 0.0, 0.0),
            ))
        } else if 400. < durability && durability <= 600. {
            Some((
                Color::rgb(0.08, 0.08, 0.08),
                Color::rgb(0.38, 0.0, 0.0),
            ))
        } else if 600. < durability && durability <= 800. {
            Some((
                Color::rgb(0.03, 0.04, 0.03),
                Color::rgb(0.42, 0.0, 0.0),
            ))
        } else {
            None
        };
        if let Some((fill, outline)) = maybe_colors {
            *draw_mode = DrawMode::Outlined {
                fill_mode: FillMode::color(fill),
                outline_mode: StrokeMode::new(outline, 4.0),
            }
        }
	}
}

pub fn handle_brick_break(
    mut commands: Commands,
    mut writer: EventWriter<audio::Event>,
    query: Query<(Entity, &Durability), Changed<Durability>>,
) {
    for (brick, durability) in query.iter() {
        if durability.0 <= 0. {
            commands.entity(brick).despawn_recursive();
            writer.send(audio::Event(brick, audio::Trigger::WallBreak, audio::Offset(-1.)))
        }
    }
}
