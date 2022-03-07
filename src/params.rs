pub struct Params {
    pub block_size: f32,
    pub block_offset: f32,
    pub map_rows: usize,
    pub map_columns: usize,
    pub bomb_count: usize,
    pub hint: bool,
    pub go_home_factor: f32,
    go_home_influence: (f32, f32),
    pub keyboard_move_speed: f32,
    pub mouse_move_speed: f32,
    pub mouse_influence: (f32, f32),
    pub bomb_velocity_threshold: f32,
}

impl Params {
    pub fn regular() -> Self {
        Self {
            block_size: 20.,
            block_offset: 35.,
            map_rows: 10,
            map_columns: 10,
            bomb_count: 5,
            hint: false,
            go_home_factor: 150. * 60.,
            go_home_influence: (0., 100.),
            keyboard_move_speed: 0.5,
            mouse_move_speed: 800.,
            mouse_influence: (50., 200.),
            bomb_velocity_threshold: 300.,
        }
    }

    pub fn chaos() -> Self {
        Self {
            block_size: 20.,
            block_offset: 35.,
            map_rows: 10,
            map_columns: 10,
            bomb_count: 5,
            hint: false,
            go_home_factor: 30.,
            go_home_influence: (100., 600.),
            keyboard_move_speed: 0.5,
            mouse_move_speed: 400.,
            mouse_influence: (100., 600.),
            bomb_velocity_threshold: 100.,
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
