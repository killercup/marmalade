use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use heron::prelude::*;
use rand::{thread_rng, Rng};

use crate::tile::{Tile, TileKind};

pub fn click_on_tile(
    mut events: EventReader<PickingEvent>,
    tiles: Query<(Entity, &Tile, &Transform)>,
    mut commands: Commands,
) {
    let mut boom = None;
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Some((e, tile, transform)) = tiles.iter().find(|(tile, ..)| e == tile) {
                if matches!(tile.kind, TileKind::Boom) {
                    info!("Boom in aile {tile:?}");
                    boom = Some(transform);
                    commands.entity(e).despawn();
                }
            }
        }
    }

    if let Some(boom) = boom {
        crate::stages::endgame(&mut commands);
        go_nuclear_from(boom, &mut commands);
    }
}

fn go_nuclear_from(source: &Transform, commands: &mut Commands) {
    let mut rng = thread_rng();

    for i in 0..200 {
        let direction = Vec3::new(
            (rng.gen_range::<i32, _>(0..2000) - 1000) as f32,
            (rng.gen_range::<i32, _>(0..2000) - 1000) as f32,
            0.0,
        );
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_translation(source.translation),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 10. })
            .insert(PhysicMaterial {
                friction: 0.1,
                density: 1000.0,
                restitution: 0.9,
            })
            .insert(Velocity::from_linear(source.translation + direction))
            .insert(RotationConstraints::lock())
            .insert(Shrapnel)
            .insert(Name::new(format!("Boom {i}")));
    }
}

#[derive(Component, Debug)]
pub struct Shrapnel;
