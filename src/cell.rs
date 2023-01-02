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
            x: 0.0, y: 0.0, size: 0.0, number: Some(1),
        }
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }

    pub fn draw(&self, text_params: &TextParams, font_x_offset: f32, font_y_offset: f32) {
        draw_rectangle(self.x, self.y, self.size, self.size, WHITE);

        if let Some(n) = self.number {
            draw_text_ex(
                n.to_string().as_str(), 
                self.x + font_x_offset, 
                self.y + font_y_offset, 
                *text_params
            );
        }
    }
}