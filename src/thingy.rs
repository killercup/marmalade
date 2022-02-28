use bevy::{core::FixedTimestep, prelude::*};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Thingy {
    pub rotation: f32,
    pub position: Vec3,
}

impl Thingy {
    pub fn nudge(&mut self, offset: Vec3) {
        self.position += offset;
    }
}

// pub fn input(mut query: Query<(&mut Transform, &mut Thingy)>) {

// }
pub fn input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Thingy)>,
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

    for (mut transform, mut thingy) in query.iter_mut() {
        let distanceFromMousPointer = Vec3::distance(Vec3::ZERO, transform.translation);
        let inverted = 1080f32 - distanceFromMousPointer;
        transform.translation += totaloffset * inverted;
    }
}
pub fn u_spin_me(mut query: Query<(&mut Transform, &mut Thingy)>) {
    for (mut transform, mut thingy) in query.iter_mut() {
        thingy.rotation = (thingy.rotation + 0.01) % 360.;
        transform.rotation = Quat::from_rotation_z(thingy.rotation);
    }
}
