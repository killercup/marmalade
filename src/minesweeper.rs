use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::PickingEvent;
use heron::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    killscreen::{GameOverEvent, KillScreen},
    map_generator::Map,
    params::Params,
    stages::GameStage,
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
    tiles: Query<(Entity, &Tile, &Transform)>,
    mut stage: ResMut<State<GameStage>>,
    mut events: EventReader<PickingEvent>,
    mut boom: EventWriter<BoomEvent>,
    mut game_over: EventWriter<GameOverEvent>,
    mut clear: EventWriter<ClearTileEvent>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Some((entity, tile, transform)) = tiles.iter().find(|(tile, ..)| e == tile) {
                if *stage.current() != GameStage::MapSet {
                    let _ = stage.set(GameStage::MapSet);
                }

                match tile.kind {
                    TileKind::Boom => {
                        info!("Boom in aisle {tile:?}");
                        crate::stages::endgame(&mut commands);
                        boom.send(BoomEvent {
                            entity,
                            source: transform.translation,
                        });
                        commands.entity(entity).despawn();
                        game_over.send(GameOverEvent);
                        return;
                    }
                    TileKind::Danger(_) => clear.send(ClearTileEvent {
                        entity,
                        tile: tile.clone(),
                    }),
                    TileKind::Fine => {
                        clear.send(ClearTileEvent {
                            entity,
                            tile: tile.clone(),
                        });
                    }
                    TileKind::Defused(_) => {}
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

    let mut existing_tiles: HashMap<usize, (Tile, Entity)> = tiles
        .iter()
        .map(|(tile, entity)| (tile.index_in_map, (tile.clone(), entity)))
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
        existing_tiles: &mut HashMap<usize, (Tile, Entity)>,
    ) {
        let neighbors = map.neighbors(me.index);
        let neighbors_we_care_about: Vec<_> = neighbors
            .iter()
            .filter(|(_coords, index, kind)| {
                existing_tiles.get(index).is_some() && *kind == TileKind::Fine
            })
            .collect();

        commands.entity(me.entity).despawn();
        existing_tiles.remove(&me.index);

        for (_coords, index, _kind) in neighbors_we_care_about {
            let (_tile, entity) = if let Some(e) = existing_tiles.get(index) {
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

        let defused_neighbors: Vec<_> = neighbors
            .iter()
            .filter(|(_coords, index, kind)| {
                existing_tiles.get(index).is_some() && matches!(*kind, TileKind::Danger(_))
            })
            .collect();

        for (_coords, index, _kind) in defused_neighbors {
            let (tile, entity) = if let Some(e) = existing_tiles.get(index) {
                e
            } else {
                continue;
            };
            commands.entity(*entity).despawn();
        }
    }
}

pub fn go_nuclear(
    mut app_state: ResMut<State<GameStage>>,
    mut events: EventReader<BoomEvent>,
    mut commands: Commands,
) {
    let mut rng = thread_rng();
    for BoomEvent { entity: _, source } in events.iter() {
        let _ = app_state.set(GameStage::KillScreen);

        for i in 0..20 {
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
}

pub fn go_nuclear_if_fast(
    params: Res<Params>,
    tiles: Query<(&Tile, &Velocity, Entity, &Transform)>,
    mut boom: EventWriter<BoomEvent>,
    mut commands: Commands,
    has_killscreen: Query<(), With<KillScreen>>,
    mut game_over: EventWriter<GameOverEvent>,
) {
    let blow_threshold = params.bomb_velocity_threshold;
    let bombs = tiles
        .iter()
        .filter(|(tile, ..)| tile.kind == TileKind::Boom);
    let fast_bombs = bombs
        .filter(|(_, velocity, ..)| velocity.linear.distance(Vec3::ZERO) > blow_threshold)
        .collect::<Vec<_>>();

    if !fast_bombs.is_empty() && has_killscreen.iter().next().is_none() {
        game_over.send(GameOverEvent);
    }

    for (_, _, entity, source) in fast_bombs {
        boom.send(BoomEvent {
            entity,
            source: source.translation,
        });
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Debug)]
pub struct Shrapnel;
