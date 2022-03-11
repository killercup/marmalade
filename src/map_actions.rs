use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use heron::prelude::*;
use nalgebra_glm::smoothstep;

use crate::{
    map_generator::Map,
    minesweeper::Shrapnel,
    params::Params,
    tile::{Tile, TileKind},
};

pub fn create_map(
    params: Res<Params>,
    asset_server: Res<AssetServer>,
    old_entities: Query<(Entity,), Or<(With<Tile>, With<Shrapnel>)>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube {
        size: params.block_size,
    }));
    let albedo = asset_server.load("graphics/TileAlbedo.png");
    let green_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(125., 0.5, 0.5),
        base_color_texture: Some(albedo),
        ..Default::default()
    });
    commands.insert_resource(Params::regular());

    for (entity,) in old_entities.iter() {
        commands.entity(entity).despawn();
    }

    let mut map = Map::new(params.map_rows, params.map_columns);
    map.set_bombs(params.bomb_count);

    let blox = (params.map_rows * params.map_columns) as f32;

    for (x, kind) in map.map.iter().enumerate() {
        let size = Vec3::new(params.block_size, params.block_size, 0.0);
        let original_position = Vec3::new(
            (x as f32 / blox.sqrt()).floor() * params.block_offset
                - blox.sqrt() * params.block_offset / 2.0,
            (x as f32 % blox.sqrt()).floor() * params.block_offset
                - blox.sqrt() * params.block_offset / 2.0,
            0.0,
        );

        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: green_tile.clone(),
                transform: Transform::from_translation(original_position),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: size / 2.,
                border_radius: None,
            })
            .insert(Velocity::from(Vec2::ZERO))
            .insert(match kind {
                TileKind::Boom => PhysicMaterial {
                    restitution: 0.2,
                    friction: 0.2,
                    density: 10.,
                },
                TileKind::Danger(x) => PhysicMaterial {
                    restitution: 0.9,
                    friction: 0.2,
                    density: 1. + 2. * smoothstep(1., 8., *x as f32),
                },
                TileKind::Fine => PhysicMaterial {
                    restitution: 0.9,
                    friction: 0.2,
                    density: 1.,
                },
                TileKind::Defused(x) => PhysicMaterial {
                    restitution: 0.9,
                    friction: 0.2,
                    density: 1. + 2. * smoothstep(1., 8., *x as f32),
                },
            })
            .insert(Damping::from_linear(0.1).with_angular(0.9))
            .insert(Tile {
                original_position,
                kind: *kind,
                index_in_map: x,
            })
            .insert_bundle(PickableBundle::default())
            .insert(Name::new(format!("My block {x}")));
    }

    commands.insert_resource(map);
}

pub fn toggle_hint(
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut params: ResMut<Params>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Tile, &mut Handle<StandardMaterial>)>,
) {
    if !keys.just_pressed(KeyCode::H) {
        return;
    }
    params.hint = !params.hint;

    let albedo = asset_server.load("graphics/TileAlbedo.png");

    let green_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(125., 0.5, 0.5),
        base_color_texture: Some(albedo.clone()),
        ..Default::default()
    });
    let red_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(15., 0.5, 0.5),
        base_color_texture: Some(albedo.clone()),
        ..Default::default()
    });
    let orange_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(55., 0.5, 0.5),
        base_color_texture: Some(albedo),
        ..Default::default()
    });

    for (tile, mut material) in query.iter_mut() {
        if !params.hint {
            *material = green_tile.clone();
            continue;
        }

        match tile.kind {
            TileKind::Boom => {
                *material = red_tile.clone();
            }
            TileKind::Danger(_) => {
                *material = orange_tile.clone();
            }
            TileKind::Defused(_) => {
                *material = green_tile.clone();
            }
            TileKind::Fine => {
                *material = green_tile.clone();
            }
        }
    }
}
