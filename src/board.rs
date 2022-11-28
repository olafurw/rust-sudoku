use macroquad::text::TextParams;

use crate::cell::Cell;
use crate::PADDING;
use crate::index_to_2d;

pub struct Board {
    pub cells: Vec<Cell>,
    pub board_size: f32,
    pub cell_size: f32,
    pub font_size: u16,
    pub font_x_offset: f32,
    pub font_y_offset: f32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::new(); 81],
            board_size: 0.0,
            cell_size: 0.0,
            font_size: 0,
            font_x_offset: 0.0,
            font_y_offset: 0.0,
        }
    }
    
    pub fn update(&mut self, board_size: f32) -> bool {
        if self.board_size as i32 == board_size as i32 {
            return false;
        }

        self.board_size = board_size;
        self.cell_size = self.board_size / 9.0;

        for (i, cell) in self.cells.iter_mut().enumerate() {
            let (x, y) = index_to_2d(i);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }

    pub fn update_font_offset(&mut self, font_width: f32, font_height: f32) {
        self.font_x_offset = (self.cell_size / 2.0) - (font_width / 2.0);
        self.font_y_offset = (self.cell_size / 2.0) + (font_height / 2.0);
    }

    pub fn draw(&self, text_params: &TextParams) {
        for cell in self.cells.iter() {
            cell.draw(text_params, self.font_x_offset, self.font_y_offset);
        }
    }
}