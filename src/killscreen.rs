use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct KillScreen;

pub struct GameOverEvent;

pub fn draw(
    asset_server: Res<AssetServer>,
    mut events: EventReader<GameOverEvent>,
    mut commands: Commands,
) {
    if events.iter().next().is_none() {
        return;
    }

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
            // Use the `Text::with_section` constructor
            text: Text {
                sections: vec![
                    TextSection {
                        value: "B O O M".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "Press R to restart".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
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
