use bevy::{input::mouse::MouseMotion, prelude::*};
use heron::{PhysicMaterial, Velocity};

use crate::params::Params;

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Tile {
    pub original_position: Vec3,
    pub kind: TileKind,
    pub index_in_map: usize,
}

#[derive(Component, Reflect, Debug, Copy, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub enum TileKind {
    /// Is a bomb
    Boom,
    /// Is a bomb neighbor
    Danger(u8),
    /// Touched and cleared
    Defused(u8),
    /// Empty tile
    Fine,
}

impl Default for TileKind {
    fn default() -> Self {
        Self::Fine
    }
}

pub fn input(
    windows: Res<Windows>,
    frame_time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    params: Res<Params>,
    mut query: Query<(&mut Transform,), With<Tile>>,
) {
    let window = windows.get_primary().unwrap();

    let mut totaloffset = Vec3::ZERO;
    let move_speed = params.keyboard_move_speed;

    if keys.pressed(KeyCode::Left) {
        totaloffset += Vec3::new(-1., 0., 0.);
    }
    if keys.pressed(KeyCode::Right) {
        totaloffset += Vec3::new(1., 0., 0.);
    }
    if keys.pressed(KeyCode::Up) {
        totaloffset += Vec3::new(0., 1., 0.);
    }
    if keys.pressed(KeyCode::Down) {
        totaloffset += Vec3::new(0., -1., 0.);
    }
    totaloffset = totaloffset * move_speed * frame_time.delta_seconds();

    for (mut transform,) in query.iter_mut() {
        let distance_from_center = Vec3::distance(Vec3::ZERO, transform.translation);
        let inverted = window.width() - distance_from_center;
        transform.translation += totaloffset * inverted;
    }
}

pub fn mouse_input(
    windows: Res<Windows>,
    frame_time: Res<Time>,
    params: Res<Params>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Velocity, &Transform, &PhysicMaterial), With<Tile>>,
) {
    let mut totaloffset = Vec3::ZERO;

    for ev in motion.iter() {
        totaloffset += Vec3::new(ev.delta.x, -ev.delta.y, 0.);
    }

    let window = windows.get_primary().unwrap();
    let cursor_position = if let Some(pos) = window.cursor_position() {
        Vec3::new(
            pos.x - window.width() / 2.,
            pos.y - window.height() / 2.,
            0.,
        )
    } else {
        Vec3::ZERO
    };

    totaloffset = totaloffset * params.mouse_move_speed * frame_time.delta_seconds();

    for (mut velocity, transform, physics_material) in query.iter_mut() {
        let distance_from_mouse_pointer = Vec3::distance(cursor_position, transform.translation);
        let influence = params.mouse_influence(distance_from_mouse_pointer);
        let acceleration = totaloffset * influence / physics_material.density;
        *velocity = velocity.with_linear(velocity.linear + acceleration);
    }
}

pub fn go_home(
    frame_time: Res<Time>,
    params: Res<Params>,
    mut query: Query<(&mut Velocity, &Transform, &Tile)>,
) {
    let damping_factor = 0.5;
    for (mut velocity, transform, thingy) in query.iter_mut() {
        let distance = thingy.original_position.distance(transform.translation);
        if distance < 0.001 {
            continue;
        }

        let influence = params.go_home_influence(distance);
        let direction = Vec3::normalize(thingy.original_position - transform.translation);
        let damping = velocity.linear * -1.0 * damping_factor;
        let acceleration =
            damping + direction * influence * frame_time.delta_seconds() * params.go_home_factor;
        *velocity = velocity.with_linear(velocity.linear + acceleration);
    }
}
