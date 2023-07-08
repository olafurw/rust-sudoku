use std::cmp::min;

use crate::board::Board;
use crate::fonts::{
    CellFont, CellPencilFont, IconFont, MenuNumberFont, ModalDifficultyFont, ModalVictoryFont,
};
use crate::generate::{create_puzzle, generate_board};
use crate::index::xy_to_index;
use crate::menu::{is_menu_action_number, Menu, MenuActions};
use crate::new_game_modal::NewGameModal;
use crate::save::{load, save};
use crate::victory_modal::VictoryModal;
use crate::{
    CELL_TEXT_COLOR, CELL_TEXT_INITIAL_COLOR, MODAL_DIFFICULTY_ONE, MODAL_DIFFICULTY_THREE,
    MODAL_DIFFICULTY_TWO, MODAL_VICTORY_GOLD, MODAL_VICTORY_RED,
};

use macroquad::prelude::*;

pub struct Context {
    pub initial_font: CellFont,
    pub font: CellFont,
    pub icon_font: IconFont,
    pub icon_font_selected: IconFont,
    pub pencil_font: CellPencilFont,
    pub menu_number_font: MenuNumberFont,
    pub menu_number_font_selected: MenuNumberFont,
    pub modal_difficulty_font_1: ModalDifficultyFont,
    pub modal_difficulty_font_2: ModalDifficultyFont,
    pub modal_difficulty_font_3: ModalDifficultyFont,
    pub modal_difficulty_title_font: ModalDifficultyFont,
    pub modal_victory_star_font: ModalVictoryFont,
    pub modal_victory_heart_font: ModalVictoryFont,
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
        let mut c = Context {
            initial_font: CellFont::new(font_path, CELL_TEXT_INITIAL_COLOR).await,
            font: CellFont::new(font_path, CELL_TEXT_COLOR).await,
            icon_font: IconFont::new(icon_font_path, BLACK).await,
            icon_font_selected: IconFont::new(icon_font_path, WHITE).await,
            pencil_font: CellPencilFont::new(font_path).await,
            menu_number_font: MenuNumberFont::new(font_path, BLACK).await,
            menu_number_font_selected: MenuNumberFont::new(font_path, WHITE).await,
            modal_difficulty_font_1: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_ONE,
            )
            .await,
            modal_difficulty_font_2: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_TWO,
            )
            .await,
            modal_difficulty_font_3: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_THREE,
            )
            .await,
            modal_difficulty_title_font: ModalDifficultyFont::new(icon_font_path, 1.0, BLACK).await,
            modal_victory_star_font: ModalVictoryFont::new(icon_font_path, 1.5, MODAL_VICTORY_GOLD)
                .await,
            modal_victory_heart_font: ModalVictoryFont::new(icon_font_path, 1.5, MODAL_VICTORY_RED)
                .await,
            board: Board::new(),
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

        let initial = load("initial");
        debug!("initial: {:?}", initial);

        let mut board = [[0; 9]; 9];
        generate_board(&mut board);
        create_puzzle(&mut board, 2);
        for (y, row) in board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                c.board.cell_state[xy_to_index(x, y, 9)].set_initial_number(*col);
            }
        }
        c.board.update_number_count();

        save("initial", c.board.cell_initial_to_string().as_str());

        c
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
                } else if menu_action == MenuActions::Pencil {
                    self.board.disable_delete_mode();
                    self.board.toggle_pencil_mode();
                } else if menu_action == MenuActions::Delete {
                    self.board.toggle_delete_mode()
                } else if menu_action == MenuActions::Undo {
                    self.board.disable_delete_mode();
                    self.board.undo();
                } else if menu_action == MenuActions::New {
                    self.new_game_modal.show();
                }
                return;
            }

            self.board.click(mouse_x, mouse_y);
            if self.board.is_victory() {
                self.victory_modal.show();
            }
        }

        let key_pressed = get_last_key_pressed();
        if let Some(key) = key_pressed {
            if key == KeyCode::U {
                self.board.undo();
            }
            return;
        }

        let char_pressed = get_char_pressed();
        if let Some(key @ '1'..='9') = char_pressed {
            let number = key as u8 - 48; // 48 = '0'
            if self.board.is_number_done(number) {
                return;
            }

            self.board.set_selected_number(number);
            self.board.highlight();
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
                    self.board = Board::new();

                    let mut board = [[0; 9]; 9];
                    generate_board(&mut board);
                    create_puzzle(&mut board, difficulty);
                    for (y, row) in board.iter().enumerate() {
                        for (x, col) in row.iter().enumerate() {
                            self.board.cell_state[xy_to_index(x, y, 9)].set_initial_number(*col);
                        }
                    }
                    self.board.update_number_count();
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

        self.initial_font.update(self.board.cell_size);
        self.font.update(self.board.cell_size);
        self.pencil_font.update(self.board.cell_size);
        self.menu_number_font.update(self.board.cell_size);
        self.menu_number_font_selected.update(self.board.cell_size);
        self.icon_font.update(self.board.cell_size);
        self.icon_font_selected.update(self.board.cell_size);
        self.menu
            .update(self.board_size, self.game_padding, self.portrait);

        self.modal_difficulty_font_1.update(self.board.cell_size);
        self.modal_difficulty_font_2.update(self.board.cell_size);
        self.modal_difficulty_font_3.update(self.board.cell_size);
        self.modal_victory_heart_font.update(self.board.cell_size);
        self.modal_victory_star_font.update(self.board.cell_size);
        self.modal_difficulty_title_font
            .update(self.board.cell_size);
        self.new_game_modal.update(
            self.game_square,
            self.modal_difficulty_font_1.width,
            self.modal_difficulty_font_1.height,
        );
        self.victory_modal.update(
            self.game_square,
            self.modal_difficulty_font_1.width,
            self.modal_difficulty_font_1.height,
        );
    }
}
