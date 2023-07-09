use std::cmp::min;

use crate::board::Board;
use crate::fonts::font_context::FontContext;
use crate::generate::{create_puzzle, generate_board};
use crate::index::xy_to_index;
use crate::menu::{is_menu_action_number, Menu, MenuActions};
use crate::new_game_modal::NewGameModal;
use crate::save::{load, save};
use crate::victory_modal::VictoryModal;

use macroquad::prelude::*;

fn load_board_from_save() -> Option<Board> {
    let loaded_board_str = load("board");
    if let Some(board_str) = loaded_board_str {
        let board_result = serde_json::from_str::<Board>(&board_str);
        if let Ok(board) = board_result {
            return Some(board);
        }
    }

    None
}

fn generate_new_board(difficulty: u8) -> Board {
    let mut board = Board::new();

    let mut board_gen = [[0; 9]; 9];
    generate_board(&mut board_gen);
    create_puzzle(&mut board_gen, difficulty);
    for (y, row) in board_gen.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            board.cell_state[xy_to_index(x, y, 9)].set_initial_number(*col);
        }
    }
    board.update_number_count();

    board
}

pub struct Context {
    pub font_context: FontContext,
    pub board: Board,
    pub menu: Menu,
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
    pub new_game_modal: NewGameModal,
    pub victory_modal: VictoryModal,
}

impl Context {
    pub async fn new(font_path: &str, icon_font_path: &str) -> Self {
        let board_save = load_board_from_save();
        let board = if let Some(board) = board_save {
            board
        } else {
            generate_new_board(2)
        };

        let context = Context {
            font_context: FontContext::new(font_path, icon_font_path).await,
            board,
            menu: Menu::new(),
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
            new_game_modal: Default::default(),
            victory_modal: Default::default(),
        };

        context.save_board();

        context
    }

    fn save_board(&self) {
        let save_data = serde_json::to_string(&self.board).unwrap();
        save("board", save_data.as_str());
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if let Some(menu_action) = self.menu.click(mouse_x, mouse_y) {
                if is_menu_action_number(menu_action) {
                    let number = menu_action as u8;
                    if self.board.is_number_done(number) {
                        return;
                    }

                    self.board.disable_delete_mode();
                    self.board.set_selected_number(number);
                    self.board.highlight();
                    self.save_board();
                } else if menu_action == MenuActions::Pencil {
                    self.board.disable_delete_mode();
                    self.board.toggle_pencil_mode();
                    self.save_board();
                } else if menu_action == MenuActions::Delete {
                    self.board.toggle_delete_mode();
                    self.save_board();
                } else if menu_action == MenuActions::Undo {
                    self.board.disable_delete_mode();
                    self.board.undo();
                    self.save_board();
                } else if menu_action == MenuActions::New {
                    self.new_game_modal.show();
                }
                return;
            }

            self.board.click(mouse_x, mouse_y);
            if self.board.is_victory() {
                self.victory_modal.show();
            }
            self.save_board();
        }

        let key_pressed = get_last_key_pressed();
        if let Some(key) = key_pressed {
            if key == KeyCode::U {
                self.board.disable_delete_mode();
                self.board.undo();
                self.save_board();
            }
            return;
        }

        let char_pressed = get_char_pressed();
        if let Some(key @ '1'..='9') = char_pressed {
            let number = key as u8 - 48; // 48 = '0'
            if self.board.is_number_done(number) {
                return;
            }

            self.board.disable_delete_mode();
            self.board.set_selected_number(number);
            self.board.highlight();
            self.save_board();
        }
    }

    pub fn update(&mut self) {
        let mut force_update = false;

        if self.victory_modal.show {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if self.victory_modal.click_outside(mouse_x, mouse_y) {
                    self.victory_modal.hide();
                    return;
                }
            }
        } else if self.new_game_modal.show {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if self.new_game_modal.click_outside(mouse_x, mouse_y) {
                    self.new_game_modal.hide();
                    return;
                }

                if let Some(difficulty) = self.new_game_modal.click(mouse_x, mouse_y) {
                    force_update = true;
                    self.new_game_modal.hide();

                    self.board = generate_new_board(difficulty);
                    self.save_board();
                }
            }
        } else {
            self.handle_input();
        }

        self.height = screen_height();
        self.width = screen_width();

        if !force_update
            && self.height as u32 == self.old_height
            && self.width as u32 == self.old_width
        {
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

        self.menu
            .update(self.board_size, self.game_padding, self.portrait);

        self.font_context.update(self.board.cell_size);

        self.new_game_modal.update(
            self.game_square,
            self.font_context.modal_difficulty_font_1.width,
            self.font_context.modal_difficulty_font_1.height,
        );
        self.victory_modal.update(
            self.game_square,
            self.font_context.modal_difficulty_font_1.width,
            self.font_context.modal_difficulty_font_1.height,
        );
    }
}
