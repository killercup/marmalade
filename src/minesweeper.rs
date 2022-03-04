use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::PickingEvent;
use heron::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    map_generator::Map,
    tile::{Tile, TileKind},
};

#[derive(Debug)]
pub struct BoomEvent {
    pub entity: Entity,
    pub source: Vec3,
}

#[derive(Debug)]
pub struct ClearTileEvent {
    pub entity: Entity,
    pub tile: Tile,
}

pub fn click_on_tile(
    mut events: EventReader<PickingEvent>,
    tiles: Query<(Entity, &Tile, &Transform)>,
    mut boom: EventWriter<BoomEvent>,
    mut clear: EventWriter<ClearTileEvent>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Some((entity, tile, transform)) = tiles.iter().find(|(tile, ..)| e == tile) {
                match tile.kind {
                    TileKind::Boom => {
                        info!("Boom in aisle {tile:?}");
                        boom.send(BoomEvent {
                            entity,
                            source: transform.translation,
                        });
                        return;
                    }
                    TileKind::Danger(_) => warn!("danger not implemented!"),
                    TileKind::Fine => {
                        clear.send(ClearTileEvent {
                            entity,
                            tile: tile.clone(),
                        });
                    }
                }
            }
        }
    }
}

pub fn clear(
    map: Res<Map>,
    mut events: EventReader<ClearTileEvent>,
    mut commands: Commands,
    tiles: Query<(&Tile, Entity)>,
) {
    let mut events = events.iter();
    let ClearTileEvent { entity, tile } = if let Some(x) = events.next() {
        x
    } else {
        return;
    };
    let remaining = events.count();
    if remaining > 0 {
        warn!("gonna ignore {remaining} events");
    }

    #[derive(Debug)]
    struct Thingy {
        pub index: usize,
        pub entity: Entity,
    }

    let mut existing_tiles: HashMap<usize, Entity> = tiles
        .iter()
        .map(|(tile, entity)| (tile.index_in_map, entity))
        .collect();

    let me = Thingy {
        entity: *entity,
        index: tile.index_in_map,
    };

    clear_neighbors(&map, me, &mut commands, &mut existing_tiles);

    fn clear_neighbors(
        map: &Map,
        me: Thingy,
        commands: &mut Commands,
        existing_tiles: &mut HashMap<usize, Entity>,
    ) {
        let neighbors = map.neighbors(me.index);
        let neighbors_we_care_about: Vec<_> = neighbors
            .iter()
            .filter(|(_coords, index, kind)| {
                existing_tiles.get(index).is_some() && *kind == TileKind::Fine
            })
            .collect();

        info!(
            "clearing {me:?}, found {} neighbors, {} are fine",
            neighbors.len(),
            neighbors_we_care_about.len()
        );
        commands.entity(me.entity).despawn();
        existing_tiles.remove(&me.index);

        for (_coords, index, _kind) in neighbors_we_care_about {
            let entity = if let Some(e) = existing_tiles.get(index) {
                e
            } else {
                continue;
            };
            clear_neighbors(
                map,
                Thingy {
                    index: *index,
                    entity: *entity,
                },
                commands,
                existing_tiles,
            );
        }
    }
}

pub fn go_nuclear(mut events: EventReader<BoomEvent>, mut commands: Commands) {
    let mut events = events.iter();
    let BoomEvent { entity, source } = if let Some(x) = events.next() {
        x
    } else {
        return;
    };
    let remaining = events.count();
    if remaining > 0 {
        warn!("gonna ignore {remaining} events");
    }

    crate::stages::endgame(&mut commands);
    commands.entity(*entity).despawn();

    let mut rng = thread_rng();

    for i in 0..200 {
        let direction = Vec3::new(
            (rng.gen_range::<i32, _>(0..2000) - 1000) as f32,
            (rng.gen_range::<i32, _>(0..2000) - 1000) as f32,
            0.0,
        );
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_translation(*source),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 10. })
            .insert(PhysicMaterial {
                friction: 0.1,
                density: 1000.0,
                restitution: 0.9,
            })
            .insert(Velocity::from_linear(*source + direction))
            .insert(RotationConstraints::lock())
            .insert(Shrapnel)
            .insert(Name::new(format!("Boom {i}")));
    }
}

#[derive(Component, Debug)]
pub struct Shrapnel;
