use bevy::prelude::*;

use crate::{
    stages::GameStage,
    tile::{Tile, TileKind},
};

#[derive(Debug, Component)]
pub struct WinScreen;

pub fn draw(asset_server: Res<AssetServer>, mut commands: Commands, query: Query<(&Tile,)>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let has_yellows = query
        .iter()
        .any(|(tile,)| matches!(tile.kind, TileKind::Danger(_)));

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
                sections: vec![
                    TextSection {
                        value: "YOU WIN\n".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: if has_yellows {
                            "Thanks for being careful".to_string()
                        } else {
                            "Please enjoy life".to_string()
                        },
                        style: TextStyle {
                            font,
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
        .insert(WinScreen);
}

pub fn hide(win_screen: Query<(Entity,), With<WinScreen>>, mut commands: Commands) {
    win_screen.for_each(|(e,)| commands.entity(e).despawn());
}

pub fn you_win(mut state: ResMut<State<GameStage>>, query: Query<(&Tile,)>) {
    if *state.current() == GameStage::KillScreen {
        return;
    }
    if query.iter().all(|(tile,)| tile.kind != TileKind::Fine) {
        let _ = state.set(GameStage::WinScreen);
    }
}
