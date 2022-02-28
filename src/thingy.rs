use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Thingy {
    pub rotation: f32,
    pub position: Vec3,
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform,), With<Thingy>>,
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
    mut query: Query<(&mut Transform,), With<Thingy>>,
    frame_time: Res<Time>,
) {
    let mut totaloffset = Vec3::ZERO;
    let move_speed = 10.;

    for ev in motion.iter() {
        // info!("{ev:?}");
        totaloffset += Vec3::new(ev.delta.x, ev.delta.y, 0.);
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
    // info!("{cursor_position:?}, {totaloffset:?}");

    for (mut transform,) in query.iter_mut() {
        let distance_from_mouse_pointer = Vec3::distance(cursor_position, transform.translation);
        let influence = 1. - nalgebra_glm::smoothstep(100., 420., distance_from_mouse_pointer);
        // info!("{totaloffset}, {influence}");
        transform.translation += totaloffset * influence;
    }
}

pub fn u_spin_me(mut query: Query<(&mut Transform, &mut Thingy)>) {
    for (mut transform, mut thingy) in query.iter_mut() {
        thingy.rotation = (thingy.rotation + 0.04) % 360.;
        transform.rotation = Quat::from_rotation_z(thingy.rotation);
    }
}
