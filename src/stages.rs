use bevy::prelude::*;

use crate::{map_actions::create_map, params::ForceParams, tile::Tile};

#[derive(Debug, Clone, Copy)]
pub enum Stage {
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
    commands.insert_resource(Stage::KillScreen);
    commands.insert_resource(ForceParams::chaos());
}

pub fn trigger_reset(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity,), With<Tile>>,
) {
    if !keys.just_pressed(KeyCode::R) {
        return;
    }

    for (entity,) in query.iter() {
        commands.entity(entity).despawn();
    }

    create_map(commands, meshes, materials);
}
