use bevy::prelude::*;

use crate::{
    killscreen::KillScreen, map_actions::create_map, minesweeper::Shrapnel, params::Params,
    tile::Tile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn trigger_reset(
    params: Res<Params>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity,), Or<(With<Tile>, With<Shrapnel>, With<KillScreen>)>>,
) {
    if !keys.just_pressed(KeyCode::R) {
        return;
    }

    for (entity,) in query.iter() {
        commands.entity(entity).despawn();
    }

    create_map(params, asset_server, commands, meshes, materials);
}
