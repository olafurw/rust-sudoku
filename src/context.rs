use crate::board::Board;
use crate::cell_font::CellFont;
use crate::{PADDING, DIGIT_COUNT};

use macroquad::prelude::*;

pub fn index_to_2d(index: usize) -> (usize, usize) {
    (index % DIGIT_COUNT, index / DIGIT_COUNT)
}

pub struct Context {
    pub font: CellFont,
    pub board: Board,
}

impl Context {
    pub async fn new(font_path: &str) -> Self {
        Context {
            font: CellFont::new(font_path).await,
            board: Board::new(),
        }
    }

    fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.click(mouse_x, mouse_y);
        }
    }

    pub fn update(&mut self) {
        self.handle_input();

        let board_size = screen_width() - (2.0 * PADDING);
        self.board.update(board_size);
        self.font.update(self.board.cell_size);
    }
}