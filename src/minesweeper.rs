use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use heron::prelude::*;
use rand::{thread_rng, Rng};

use crate::tile::{Tile, TileKind};

#[derive(Debug)]
pub struct BoomEvent {
    pub entity: Entity,
    pub source: Vec3,
}

#[derive(Debug)]
pub struct ClearTileEvent {
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
                    TileKind::Danger(_) => todo!(),
                    TileKind::Fine => {
                        clear.send(ClearTileEvent { tile: tile.clone() });
                    }
                }
            }
        }
    }
}

pub fn go_nuclear(mut boom: EventReader<BoomEvent>, mut commands: Commands) {
    let BoomEvent { entity, source } = if let Some(x) = boom.iter().next() {
        x
    } else {
        return;
    };
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
