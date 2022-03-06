use bevy::prelude::*;

use crate::{
    killscreen::KillScreen, map_actions::create_map, minesweeper::Shrapnel, params::Params,
    startscreen::GameStartEvent, tile::Tile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameStage {
    NewGame,
    MapSet,
    KillScreen,
}

pub fn trigger_endgame(keys: Res<Input<KeyCode>>, mut commands: Commands) {
    if !keys.just_pressed(KeyCode::Q) {
        return;
    }

    endgame(&mut commands);
}

pub fn endgame(commands: &mut Commands) {
    commands.insert_resource(GameStage::KillScreen);
    commands.insert_resource(Params::chaos());
}

pub fn trigger_reset(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<GameStage>>) {
    if !keys.just_pressed(KeyCode::R) {
        return;
    }

    let _ = app_state.set(GameStage::NewGame);
}
