use bevy::prelude::*;

use crate::{map_actions::create_map, minesweeper::Shrapnel, params::Params, tile::Tile};

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
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity,), Or<(With<Tile>, With<Shrapnel>)>>,
    asset_server: Res<AssetServer>,
) {
    if !keys.just_pressed(KeyCode::R) {
        return;
    }

    for (entity,) in query.iter() {
        commands.entity(entity).despawn();
    }

    create_map(commands, meshes, materials, asset_server);
}
