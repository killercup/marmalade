use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use heron::prelude::*;

use crate::{
    map_generator::Map,
    params::Params,
    stages::GameStage,
    tile::{Tile, TileKind},
    BLOCK_OFFSET, BLOCK_SIZE, BOMB_COUNT, MAP_COLUMNS, MAP_ROWS,
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
    commands.insert_resource(GameStage::NewGame);
    commands.insert_resource(Params::regular());

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

#[derive(Debug)]
pub struct SetMapEvent;

pub fn trigger_set_map(keys: Res<Input<KeyCode>>, mut trigger: EventWriter<SetMapEvent>) {
    if !keys.just_pressed(KeyCode::X) {
        return;
    }
    trigger.send(SetMapEvent);
}

pub fn set_map(
    mut events: EventReader<SetMapEvent>,
    mut stage: ResMut<GameStage>,
    mut map: ResMut<Map>,
    mut query: Query<(&mut Tile, &mut PhysicMaterial)>,
) {
    if *stage != GameStage::NewGame {
        return;
    }
    if events.iter().next().is_none() {
        return;
    }

    map.set_bombs(BOMB_COUNT);

    for (mut tile, mut physics) in query.iter_mut() {
        tile.kind = map.map[tile.index_in_map];
        match tile.kind {
            TileKind::Boom => {
                physics.density = 50.;
                physics.restitution = 0.2;
            }
            TileKind::Danger(_) => {
                physics.density = 2.;
            }
            TileKind::Fine => {
                physics.density = 1.;
                // same old green
            }
        }
    }

    *stage = GameStage::MapSet;
    info!("Game set");
}

pub fn draw_hints(
    params: Res<Params>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Tile, &mut Handle<StandardMaterial>)>,
) {
    let green_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(125., 0.5, 0.5),
        ..Default::default()
    });
    let red_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(15., 0.5, 0.5),
        ..Default::default()
    });
    let orange_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(55., 0.5, 0.5),
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
            TileKind::Fine => {
                *material = green_tile.clone();
            }
        }
    }
}
