use crate::board::Board;
use crate::PADDING;

use macroquad::{prelude::*};

fn cell_font(font: &Font, cell_size: f32) -> u16 {
    if cell_size < 1.0 {
        return 1;
    }

    let mut font_size = 1;

    // todo: turn into binary search
    for test_size in 1..200 {
        let measurement = measure_text("9", Some(*font), test_size, 1.0);
        if measurement.height / cell_size > 0.6 {
            font_size = test_size;
            break;
        }
    }

    font_size
}

pub struct Context {
    pub params: TextParams,
    pub font_height: f32,
    pub font_width: f32,
    pub board: Board,
}

impl Context {
    pub async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        Context {
            params: TextParams { font, font_size: 48, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: BLACK },
            font_height: measure.height,
            font_width: measure.width,
            board: Board::new(),
        }
    }

    pub fn update(&mut self) {
        let board_size = screen_width() - (2.0 * PADDING);
        if !self.board.update(board_size) {
            return;
        }

        self.params.font_size = cell_font(&self.params.font, self.board.cell_size);

        let measure = measure_text("9", Some(self.params.font), self.params.font_size, 1.0);
        self.board.update_font_offset(measure.width, measure.height);
    }
}