use bevy::prelude::*;

use crate::stages::GameStage;

#[derive(Debug, Component)]
pub struct KillScreen;

pub struct GameOverEvent;

pub fn end_game(mut events: EventReader<GameOverEvent>, mut stage: ResMut<State<GameStage>>) {
    if events.iter().next().is_none() {
        return;
    }
    if *stage.current() == GameStage::WinScreen {
        return;
    }
    let _ = stage.set(GameStage::KillScreen);
}

pub fn draw(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "B O O M ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "Press R to restart".to_string(),
                        style: TextStyle {
                            font,
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
        .insert(KillScreen);
}

pub fn hide(kill_screen: Query<(Entity,), With<KillScreen>>, mut commands: Commands) {
    kill_screen.for_each(|(e,)| commands.entity(e).despawn());
}
