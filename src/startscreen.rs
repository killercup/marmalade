use bevy::prelude::*;

use crate::params::Params;

#[derive(Debug, Component)]
pub struct StartScreen;

pub struct GameStartEvent;

pub fn draw(
    asset_server: Res<AssetServer>,
    params: Res<Params>,
    mut events: EventReader<GameStartEvent>,
    mut commands: Commands,
) {
    if events.iter().next().is_none() {
        return;
    }

    let count = params.bomb_count;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "CLEAR THE FIELD".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: format!("There are {count} bombs"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "Caution: Don't shake the bombs!".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .insert(StartScreen);
}
