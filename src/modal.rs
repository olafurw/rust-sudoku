use std::cell;

use macroquad::prelude::{is_mouse_button_pressed, mouse_position, MouseButton};

use crate::menu_item::MenuItem;

pub struct Modal {
    pub show: bool,
    pub new_game: bool,
    pub x: f32,
    pub y: f32,
    pub game_square: f32,
    pub font_height: f32,
    pub width: f32,
    pub height: f32,
    pub difficulty_1: MenuItem,
    pub difficulty_2: MenuItem,
    pub difficulty_3: MenuItem,
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl Modal {
    pub fn new() -> Self {
        Modal {
            show: false,
            new_game: false,
            x: 0.0,
            y: 0.0,
            game_square: 0.0,
            font_height: 0.0,
            width: 0.0,
            height: 0.0,
            difficulty_1: Default::default(),
            difficulty_2: Default::default(),
            difficulty_3: Default::default(),
        }
    }

    pub fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if self.click_outside(mouse_x, mouse_y) {
                self.close_modal();
                return;
            }
        }
    }

    pub fn show_new_game(&mut self) {
        self.show = true;
        self.new_game = true;

        self.update_new_game();
    }

    pub fn close_modal(&mut self) {
        self.show = false;
        self.new_game = false;
    }

    pub fn update(&mut self, game_square: f32, font_height: f32) {
        self.game_square = game_square;
        self.font_height = font_height;

        if self.show && self.new_game {
            self.update_new_game();
        }
    }

    pub fn update_new_game(&mut self) {
        let cell_width = self.game_square / 9.0;
        self.width = cell_width * 5.0;
        self.x = self.game_square / 2.0 - (self.width / 2.0);
        self.height = cell_width * 2.0;
        self.y = self.game_square / 2.0 - (self.height / 2.0);

        let button_y = self.y + self.height / 2.0 - self.font_height / 2.0;

        let button_2_x = self.x + (self.width / 2.0) - self.font_height;
        self.difficulty_2.update(button_2_x, button_y, cell_width);

        let button_1_x = button_2_x - (cell_width * 1.5);
        self.difficulty_1.update(button_1_x, button_y, cell_width);

        let button_3_x = button_2_x + (cell_width * 1.5);
        self.difficulty_3.update(button_3_x, button_y, cell_width);
    }

    fn click_outside(&mut self, x: f32, y: f32) -> bool {
        x < self.x || x > self.x + self.width || y < self.y || y > self.y + self.height
    }
}
