use bevy::prelude::*;

use crate::game::level::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Default)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct ScoreboardRootNode;
#[derive(Component)]
pub struct ScoreboardNode;

const BACKGROUND: Color = Color::rgb(0.1, 0.2, 0.45);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let mut style = Style::default();
    style.position_type = PositionType::Absolute;
    style.position = Rect {
        top: Val::Percent(0.),
        left: Val::Percent(0.),
        right: Val::Percent(90.),
        bottom: Val::Percent(93.),
    };

    // ui camera
    commands
        .spawn()
        .insert(ScoreboardRootNode)
        .insert_bundle(NodeBundle {
            color: BACKGROUND.into(),
            style,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Score: ".to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    Default::default(),
                    ),
                    ..Default::default()
                });
            parent
                .spawn()
                .insert(ScoreboardNode)
                .insert_bundle(TextBundle {
                        text: Text::with_section(
                        "0".to_string(),
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
        });
}

pub fn handle_tracking_score(
    mut ui_query: Query<&mut Text, With<ScoreboardNode>>,
    res: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    if ui_query.get_single_mut().is_err() {
        return
    }
    let mut text_node = ui_query.get_single_mut().unwrap();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    (*text_node) = Text::with_section(
        res.0.to_string(),
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
        Default::default(),
    )
}
