use bevy::prelude::*;

pub struct Params {
    pub hint: bool,
    pub go_home_factor: f32,
    go_home_influence: (f32, f32),
    pub keyboard_move_speed: f32,
    pub mouse_move_speed: f32,
    pub mouse_influence: (f32, f32),
}

impl Params {
    pub fn regular() -> Self {
        Self {
            hint: false,
            go_home_factor: 150. * 60.,
            go_home_influence: (0., 100.),
            keyboard_move_speed: 0.5,
            mouse_move_speed: 800.,
            mouse_influence: (50., 200.),
        }
    }

    pub fn chaos() -> Self {
        Self {
            hint: false,
            go_home_factor: 30.,
            go_home_influence: (100., 600.),
            keyboard_move_speed: 0.5,
            mouse_move_speed: 400.,
            mouse_influence: (100., 600.),
        }
    }

    pub fn go_home_influence(&self, distance: f32) -> f32 {
        nalgebra_glm::smoothstep(self.go_home_influence.0, self.go_home_influence.1, distance)
    }

    pub fn mouse_influence(&self, distance_from_mouse_pointer: f32) -> f32 {
        1. - nalgebra_glm::smoothstep(
            self.mouse_influence.0,
            self.mouse_influence.1,
            distance_from_mouse_pointer,
        )
    }
}

pub fn toggle_hint(keys: Res<Input<KeyCode>>, mut params: ResMut<Params>) {
    if !keys.just_pressed(KeyCode::H) {
        return;
    }
    params.hint = !params.hint;
}
