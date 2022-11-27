use macroquad::prelude::*;

#[derive(Clone)]
pub struct Cell {
    x: f32,
    y: f32,
    size: f32,
    number: Option<u32>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            x: 0.0, y: 0.0, size: 0.0, number: None,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, WHITE);
        draw_text("9", self.x, self.y, 100.0, BLACK);
    }
}