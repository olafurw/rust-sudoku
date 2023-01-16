use crate::board::Board;
use crate::cell_font::CellFont;
use crate::{PADDING, DIGIT_COUNT};

use macroquad::{prelude::*};

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

    fn draw_cell_lines(&self) {
        for x in 1..9 {
            if x % 3 == 0 {
                continue;
            }
    
            let line_width = self.board.board_size * 0.0025;
            let line_width = if line_width < 0.5 { 0.5 } else { line_width };
    
            let offset = PADDING + ((x as f32 * self.board.cell_size) - (line_width / 2.0));
            draw_line(offset, PADDING, offset, self.board.board_size + PADDING, line_width, GRAY);
            draw_line(PADDING, offset, self.board.board_size + PADDING, offset, line_width, GRAY);
        }
    }
    
    fn draw_box_lines(&self) {
        for x in 1..3 {
            let line_width = self.board.board_size * 0.005;
            let line_width = if line_width < 1.0 { 1.0 } else { line_width };
    
            let offset = PADDING + ((x as f32 * (3.0 * self.board.cell_size)) - (line_width / 2.0));
            draw_line(offset, PADDING, offset, self.board.board_size + PADDING, line_width, BLACK);
            draw_line(PADDING, offset, self.board.board_size + PADDING, offset, line_width, BLACK);
        }
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        self.board.draw(&self.font);
    
        self.draw_cell_lines();
        self.draw_box_lines();
    }
}