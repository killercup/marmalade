use bevy::prelude::*;

use crate::{params::Params, stages::GameStage};

#[derive(Debug, Component)]
pub struct StartScreen;

pub struct GameStartEvent;

pub fn draw(asset_server: Res<AssetServer>, params: Res<Params>, mut commands: Commands) {
    let count = params.bomb_count;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                flex_wrap: FlexWrap::Wrap,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "CLEAR THE FIELD\n".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: format!("There are {count} bombs\n"),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .insert(StartScreen);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                flex_wrap: FlexWrap::Wrap,
                position: Rect {
                    bottom: Val::Px(15.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Caution: Don't shake the bombs!".to_string(),
                    style: TextStyle {
                        font,
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .insert(StartScreen);
}

pub fn hide(start_screen: Query<(Entity,), With<StartScreen>>, mut commands: Commands) {
    start_screen.for_each(|(e,)| commands.entity(e).despawn());
}
