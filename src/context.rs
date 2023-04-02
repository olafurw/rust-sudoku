use std::cmp::min;

use crate::board::Board;
use crate::fonts::{CellFont, CellPencilFont, IconFont, MenuNumberFont};
use crate::index::xy_to_index;
use crate::menu::{Menu, is_menu_action_number, MenuActions};
use crate::{CELL_TEXT_COLOR, CELL_TEXT_INITIAL_COLOR};

use macroquad::prelude::*;

pub struct Context {
    pub initial_font: CellFont,
    pub font: CellFont,
    pub icon_font: IconFont,
    pub pencil_font: CellPencilFont,
    pub menu_number_font: MenuNumberFont,
    pub board: Board,
    pub menu: Menu,
    pub game_padding: f32,
    pub game_square: f32,
    pub old_game_square: u32,
    pub board_size: f32,
    pub portrait: bool,
    pub demo: [[u8; 9]; 9],
}

impl Context {
    pub async fn new(font_path: &str, icon_font_path: &str) -> Self {
        Context {
            initial_font: CellFont::new(font_path, CELL_TEXT_INITIAL_COLOR).await,
            font: CellFont::new(font_path, CELL_TEXT_COLOR).await,
            icon_font: IconFont::new(icon_font_path, BLACK).await,
            pencil_font: CellPencilFont::new(font_path).await,
            menu_number_font: MenuNumberFont::new(font_path).await,
            board: Board::new(),
            menu: Menu::new(),
            game_padding: 0.0,
            game_square: 0.0,
            old_game_square: 0,
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
                        self.board.cell_state[xy_to_index(x, y, 9)].set_initial_number(*col);
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.click(mouse_x, mouse_y);
            if let Some(menu_action) = self.menu.click(mouse_x, mouse_y) {
                if is_menu_action_number(menu_action) {
                    self.board.number(menu_action as u8);
                } else if menu_action == MenuActions::Pencil {
                    self.board.toggle_pencil_mode();
                } else if menu_action == MenuActions::Undo {
                    self.board.undo();
                }
                return;
            }
        }

        let key_pressed = get_last_key_pressed();
        if let Some(key) = key_pressed {
            if key == KeyCode::Delete {
                self.board.clear_number();
            } else if key == KeyCode::U {
                self.board.undo();
            }
            return;
        }

        let char_pressed = get_char_pressed();
        if let Some(key @ '1'..='9') = char_pressed {
            let number = key as u8 - 48; // 48 = '0'
            self.board.number(number);

        }
    }

    pub fn update(&mut self) {
        self.handle_input();

        let height = screen_height();
        let width = screen_width();

        self.portrait = height >= width;
        self.game_square = min(height as u32, width as u32) as f32;

        if self.game_square as u32 == self.old_game_square {
            return;
        }
        self.old_game_square = self.game_square as u32;

        self.game_padding = self.game_square * 0.02;
        self.board_size = self.game_square - (2.0 * self.game_padding);

        self.board.update(self.board_size, self.game_padding, self.portrait);
        self.initial_font.update(self.board.cell_size);
        self.font.update(self.board.cell_size);
        self.pencil_font.update(self.board.cell_size);
        self.menu_number_font.update(self.board.cell_size);
        self.icon_font.update(self.board.cell_size);
        self.menu.update(self.board_size, self.game_padding, self.portrait);
    }
}
