use std::cmp::min;

use crate::board::Board;
use crate::cell_font::{CellFont, CellPencilFont};
use crate::PADDING;

use macroquad::prelude::*;
use macroquad::ui::Skin;

pub fn index_to_xy(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

pub fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    width * y + x
}

pub struct Context {
    pub font: CellFont,
    pub skin: Skin,
    pub pencil_font: CellPencilFont,
    pub board: Board,
    pub game_area: u32,
    pub board_size: f32,
    pub portrait: bool,
    pub demo: [[u32; 9]; 9],
}

impl Context {
    pub async fn new(font_path: &str, skin: Skin) -> Self {
        Context {
            font: CellFont::new(font_path).await,
            skin,
            pencil_font: CellPencilFont::new(font_path).await,
            board: Board::new(),
            game_area: 0,
            board_size: 0.0,
            portrait: true,
            demo: [
                [2, 0, 0, 3, 0, 6, 0, 0, 0],
                [6, 0, 5, 9, 0, 0, 4, 0, 8],
                [0, 0, 0, 0, 0, 0, 5, 0, 2],
                [4, 0, 9, 0, 6, 3, 0, 0, 0],
                [0, 0, 0, 8, 0, 0, 7, 0, 1],
                [0, 0, 1, 0, 4, 0, 0, 9, 0],
                [1, 0, 6, 2, 7, 0, 0, 0, 0],
                [0, 2, 0, 0, 0, 0, 8, 0, 4],
                [0, 0, 4, 0, 1, 8, 0, 0, 7],
            ],
        }
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Right) {
            for (y, row) in self.demo.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    if *col != 0 {
                        self.board.cells[xy_to_index(x, y, 9)].set_number(*col);
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.click(mouse_x, mouse_y);
        }

        let key_pressed = get_char_pressed();
        if let Some(key @ '1'..='9') = key_pressed {
            let number = key as u32 - '0' as u32;
            self.board.number(number);
        }
    }

    pub fn update(&mut self) {
        self.handle_input();

        let height = screen_height();
        let width = screen_width();

        self.portrait = height >= width;

        self.game_area = min(height as u32, width as u32);
        self.board_size = self.game_area as f32 - (2.0 * PADDING);

        self.board.update(self.board_size);
        self.font.update(self.board.cell_size);
        self.pencil_font.update(self.board.cell_size);
    }
}
