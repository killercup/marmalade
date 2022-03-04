pub struct ForceParams {
    pub go_home_factor: f32,
    go_home_influence: (f32, f32),
    pub keyboard_move_speed: f32,
    pub mouse_move_speed: f32,
    pub mouse_influence: (f32, f32),
}

impl ForceParams {
    pub fn regular() -> Self {
        Self {
            go_home_factor: 150.,
            go_home_influence: (0., 100.),
            keyboard_move_speed: 0.5,
            mouse_move_speed: 400.,
            mouse_influence: (3., 20.),
        }
    }

    pub fn chaos() -> Self {
        Self {
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
