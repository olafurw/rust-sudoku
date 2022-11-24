use macroquad::prelude::*;

pub struct Cell {
    number: u32,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            number: 9
        }
    }

    pub fn draw(&self) {
        draw_rectangle(0.0, 0.0, 64.0, 64.0, WHITE);
        draw_text("9", 100.0, 100.0, 100.0, BLACK);
    }
}