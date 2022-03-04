use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use heron::prelude::*;

use crate::{
    map_generator::Map,
    params::ForceParams,
    stages::Stage,
    tile::{Tile, TileKind},
    BLOCK_OFFSET, BLOCK_SIZE, MAP_COLUMNS, MAP_ROWS,
};

pub fn create_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
    let green_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(125., 0.5, 0.5),
        ..Default::default()
    });
    commands.insert_resource(Stage::NewGame);
    commands.insert_resource(ForceParams::regular());

    let map = Map::new(MAP_ROWS, MAP_COLUMNS);
    let blox = (MAP_ROWS * MAP_COLUMNS) as f32;

    for (x, kind) in map.map.iter().enumerate() {
        let size = Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0.0);
        let original_position = Vec3::new(
            (x as f32 / blox.sqrt()).floor() * BLOCK_OFFSET - blox.sqrt() * BLOCK_OFFSET / 2.0,
            (x as f32 % blox.sqrt()).floor() * BLOCK_OFFSET - blox.sqrt() * BLOCK_OFFSET / 2.0,
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
            .insert(PhysicMaterial {
                restitution: 0.9,
                friction: 0.2,
                density: 1.,
            })
            .insert(Damping::from_linear(0.5).with_angular(0.9))
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

pub fn set_map(
    keys: Res<Input<KeyCode>>,
    mut stage: ResMut<Stage>,
    mut map: ResMut<Map>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Tile, &mut Handle<StandardMaterial>)>,
) {
    if !keys.just_pressed(KeyCode::X) {
        return;
    }
    if !matches!(*stage, Stage::NewGame) {
        return;
    }

    map.set_bombs(16);

    let red_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(15., 0.5, 0.5),
        ..Default::default()
    });
    let orange_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(55., 0.5, 0.5),
        ..Default::default()
    });

    for (mut tile, mut material) in query.iter_mut() {
        tile.kind = map.map[tile.index_in_map];
        match tile.kind {
            TileKind::Boom => {
                *material = red_tile.clone();
            }
            TileKind::Danger(_) => {
                *material = orange_tile.clone();
            }
            TileKind::Fine => {
                // same old green
            }
        }
    }

    *stage = Stage::MapSet;
    info!("Game set");
}
