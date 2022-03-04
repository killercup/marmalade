use bevy::{input::mouse::MouseMotion, prelude::*};
use heron::Velocity;

#[derive(Component, Reflect, Default, Debug)]
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
    /// Empty tile
    Fine,
}

impl Default for TileKind {
    fn default() -> Self {
        Self::Fine
    }
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform,), With<Tile>>,
    frame_time: Res<Time>,
) {
    let mut totaloffset = Vec3::ZERO;
    let move_speed = 0.5;

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
        let distance_from_mouse_pointer = Vec3::distance(Vec3::ZERO, transform.translation);
        let inverted = 1080f32 - distance_from_mouse_pointer;
        transform.translation += totaloffset * inverted;
    }
}

pub fn mouse_input(
    mut motion: EventReader<MouseMotion>,
    windows: Res<Windows>,
    mut query: Query<(&mut Velocity, &Transform), With<Tile>>,
    frame_time: Res<Time>,
) {
    let mut totaloffset = Vec3::ZERO;
    let move_speed = 400.;

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

    totaloffset = totaloffset * move_speed * frame_time.delta_seconds();

    for (mut velocity, transform) in query.iter_mut() {
        let distance_from_mouse_pointer = Vec3::distance(cursor_position, transform.translation);
        let influence = 1. - nalgebra_glm::smoothstep(3., 20., distance_from_mouse_pointer);

        *velocity = velocity.with_linear((velocity.linear + totaloffset) * influence);
    }
}

pub fn go_home(params: Res<ForceParams>, mut query: Query<(&mut Velocity, &Transform, &Tile)>) {
    for (mut velocity, transform, thingy) in query.iter_mut() {
        let distance = thingy.original_position.distance(transform.translation);
        if distance < 0.001 {
            continue;
        }

        let influence = params.go_home_influence(distance);
        let direction = Vec3::normalize(thingy.original_position - transform.translation);
        let force_mult = params.go_home_factor;
        *velocity = velocity.with_linear(velocity.linear + direction * influence * force_mult);
    }
}

pub struct ForceParams {
    pub go_home_factor: f32,
    go_home_influence: (f32, f32),
}

impl ForceParams {
    pub fn go_home_influence(&self, distance: f32) -> f32 {
        nalgebra_glm::smoothstep(
            self.go_home_influence.0,
            self.go_home_influence.1,
            distance.sqrt(),
        )
    }

    pub fn regular() -> Self {
        Self {
            go_home_factor: 150.,
            go_home_influence: (0., 10.),
        }
    }
}
