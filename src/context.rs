use std::cmp::min;

use crate::board::Board;
use crate::fonts::{CellFont, CellPencilFont, IconFont, MenuNumberFont};
use crate::generate::{create_puzzle, generate_board};
use crate::index::xy_to_index;
use crate::menu::{is_menu_action_number, Menu, MenuActions};
use crate::{CELL_TEXT_COLOR, CELL_TEXT_INITIAL_COLOR};

use macroquad::prelude::*;

pub struct Context {
    pub initial_font: CellFont,
    pub font: CellFont,
    pub icon_font: IconFont,
    pub pencil_font: CellPencilFont,
    pub menu_number_font: MenuNumberFont,
    pub menu_number_font_selected: MenuNumberFont,
    pub board: Board,
    pub menu: Menu,
    pub selected_number: Option<u8>,
    pub game_padding: f32,
    pub width_padding: f32,
    pub height_padding: f32,
    pub game_square: f32,
    pub board_size: f32,
    pub height: f32,
    pub width: f32,
    pub old_height: u32,
    pub old_width: u32,
    pub portrait: bool,
}

impl Context {
    pub async fn new(font_path: &str, icon_font_path: &str) -> Self {
        let mut c = Context {
            initial_font: CellFont::new(font_path, CELL_TEXT_INITIAL_COLOR).await,
            font: CellFont::new(font_path, CELL_TEXT_COLOR).await,
            icon_font: IconFont::new(icon_font_path, BLACK).await,
            pencil_font: CellPencilFont::new(font_path).await,
            menu_number_font: MenuNumberFont::new(font_path, BLACK).await,
            menu_number_font_selected: MenuNumberFont::new(font_path, WHITE).await,
            board: Board::new(),
            menu: Menu::new(),
            selected_number: None,
            width_padding: 0.0,
            game_padding: 0.0,
            height_padding: 0.0,
            game_square: 0.0,
            board_size: 0.0,
            height: 0.0,
            width: 0.0,
            old_height: 0,
            old_width: 0,
            portrait: true,
        };

        let mut board = [[0; 9]; 9];
        generate_board(&mut board);
        create_puzzle(&mut board, 2);
        for (y, row) in board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                c.board.cell_state[xy_to_index(x, y, 9)].set_initial_number(*col);
            }
        }

        c
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.board.new_click(mouse_x, mouse_y);
            if let Some(menu_action) = self.menu.click(mouse_x, mouse_y) {
                if is_menu_action_number(menu_action) {
                    self.selected_number = Some(menu_action as u8);
                    self.board.set_selected_number(menu_action as u8);
                    self.board.highlight();
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

        self.height = screen_height();
        self.width = screen_width();

        if self.height as u32 == self.old_height && self.width as u32 == self.old_width {
            return;
        }
        self.old_height = self.height as u32;
        self.old_width = self.width as u32;

        self.portrait = self.height >= self.width;
        self.game_square = min(self.height as u32, self.width as u32) as f32;
        let menu_size = self.game_square / 3.0;

        if self.portrait {
            let missing = self.height - (self.game_square + menu_size);
            if missing < 0.0 {
                self.game_square += missing;
            }
        } else {
            let missing = self.width - (self.game_square + menu_size);
            if missing < 0.0 {
                self.game_square += missing;
            }
        }

        self.game_padding = self.game_square * 0.02;

        if self.portrait {
            let padding = self.width - self.game_square;
            self.width_padding = self.game_padding + (padding / 2.0);
            self.height_padding = self.game_padding;
        } else {
            let padding = self.height - self.game_square;
            self.width_padding = self.game_padding;
            self.height_padding = self.game_padding + (padding / 2.0);
        }

        self.board_size = self.game_square - (2.0 * self.game_padding);

        self.board
            .update(self.board_size, self.game_padding, self.portrait);

        self.initial_font.update(self.board.cell_size);
        self.font.update(self.board.cell_size);
        self.pencil_font.update(self.board.cell_size);
        self.menu_number_font.update(self.board.cell_size);
        self.menu_number_font_selected.update(self.board.cell_size);
        self.icon_font.update(self.board.cell_size);
        self.menu
            .update(self.board_size, self.game_padding, self.portrait);
    }
}
