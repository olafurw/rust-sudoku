use macroquad::prelude::{is_mouse_button_pressed, MouseButton};

pub struct Modal {
    pub show: bool,
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl Modal {
    pub fn new() -> Self {
        Modal { show: false }
    }

    pub fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.show = false;
        }
    }

    pub fn update(&mut self, board_size: f32) {}
}
