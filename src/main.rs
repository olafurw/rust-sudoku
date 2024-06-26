// TODO
//
// different hightlight mode switcher
// add saving through wasm quad_storage
// refactor fonts and put them in their own file to use in context.rs
// screen and game sizing should be in one place, currently it's in context and board.
// when shrinking due to space missing, center the game board
// move the menu to the other side in landscape
// add WAY more tests
// add menu and options
// add win screen
// move input handling out of context and into its own file.
// put some of these common files into their own folder, like draw

mod board;
mod board_history;
mod cell_location;
mod cell_state;
mod context;
mod draw;
mod fonts;
mod generate;
mod index;
mod menu;
mod menu_item;
mod new_game_modal;
mod victory_modal;

#[cfg_attr(target_arch = "wasm32", path = "save_wasm.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "save_win.rs")]
mod save;

use context::Context;
use draw::draw_context::draw_context;
//use egui_macroquad::egui;
use macroquad::prelude::*;

pub const DIGIT_COUNT: u8 = 9;
pub const CELL_COLOR_NORMAL: Color = color_u8!(255, 255, 255, 255);
pub const CELL_COLOR_NORMAL_EMPHASIZE: Color = color_u8!(186, 209, 255, 255);
pub const CELL_COLOR_PENCIL_EMPHASIZE: Color = color_u8!(255, 193, 140, 255);
pub const CELL_COLOR_HIGHLIGHTED: Color = color_u8!(219, 219, 219, 255);

pub const CELL_TEXT_COLOR: Color = color_u8!(41, 91, 135, 255);
pub const CELL_TEXT_INITIAL_COLOR: Color = color_u8!(0, 0, 0, 255);

pub const MENU_NUMBER_BACKGROUND_NORMAL: Color = color_u8!(56, 76, 107, 255);
pub const MENU_NUMBER_BACKGROUND_PENCIL: Color = color_u8!(226, 138, 43, 255);
pub const MENU_DELETE_BACKGROUND: Color = color_u8!(255, 0, 0, 255);

pub const MODAL_BACKGROUND: Color = color_u8!(0, 0, 0, 128);
pub const MODAL_DIFFICULTY_ONE: Color = color_u8!(0, 128, 0, 255);
pub const MODAL_DIFFICULTY_TWO: Color = color_u8!(255, 128, 0, 255);
pub const MODAL_DIFFICULTY_THREE: Color = color_u8!(255, 0, 0, 255);
pub const MODAL_VICTORY_GOLD: Color = color_u8!(242, 183, 5, 255);
pub const MODAL_VICTORY_RED: Color = color_u8!(239, 50, 50, 255);

pub const DEBUG_RED: Color = color_u8!(255, 0, 0, 128);
pub const DEBUG_BLUE: Color = color_u8!(0, 0, 255, 128);

pub const BOX_INDEXES: &[[usize; 9]; 9] = &[
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

pub const ROW_INDEXES: &[[usize; 9]; 9] = &[
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];

pub const COLUMN_INDEXES: &[[usize; 9]; 9] = &[
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
];

pub const ICON_UNDO: &str = "\u{e166}";
pub const ICON_PENCIL: &str = "\u{e3c9}";
pub const ICON_PENCIL_SLASH: &str = "\u{e950}";
pub const ICON_NEW: &str = "\u{e146}";
pub const ICON_DIFFICULTY_1: &str = "\u{f784}";
pub const ICON_DIFFICULTY_2: &str = "\u{f783}";
pub const ICON_DIFFICULTY_3: &str = "\u{f782}";
pub const ICON_DIFFICULTY_NEW: &str = "\u{e3fc}";
pub const ICON_DELETE: &str = "\u{e872}";
pub const ICON_VICTORY_HEART: &str = "\u{e87d}";
pub const ICON_VICTORY_STAR: &str = "\u{e8d0}";

pub fn is_legal_number(number: u8) -> bool {
    (1..=9).contains(&number)
}

pub fn is_legal_index(number: usize) -> bool {
    (0..=80).contains(&number)
}

#[macroquad::main("Sudoku")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let mut context = Context::new(
        "liberation-sans-minimized.ttf",
        "material-font-minimized.ttf",
    )
    .await;

    request_new_screen_size(800.0, 1080.0);

    loop {
        context.update();
        draw_context(&context);

        /*egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_visuals(egui::Visuals::light());
            egui::Window::new("debug").show(egui_ctx, |ui| {
                ui.label(format!("mode: {:?}", context.board.mode));
            });
        });

        egui_macroquad::draw();*/

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use crate::{is_legal_index, is_legal_number};

    #[test]
    fn legal_number_test() {
        assert!(!is_legal_number(0));
        assert!(is_legal_number(1));
        assert!(is_legal_number(2));
        assert!(is_legal_number(3));
        assert!(is_legal_number(4));
        assert!(is_legal_number(5));
        assert!(is_legal_number(6));
        assert!(is_legal_number(7));
        assert!(is_legal_number(8));
        assert!(is_legal_number(9));
        assert!(!is_legal_number(10));
        assert!(!is_legal_number(11));
    }

    #[test]
    fn legal_index_test() {
        assert!(is_legal_index(0));
        assert!(is_legal_index(1));
        assert!(is_legal_index(2));
        assert!(is_legal_index(3));
        assert!(is_legal_index(4));
        assert!(is_legal_index(5));
        assert!(is_legal_index(6));
        assert!(is_legal_index(7));
        assert!(is_legal_index(8));
        assert!(is_legal_index(9));
        assert!(is_legal_index(78));
        assert!(is_legal_index(79));
        assert!(is_legal_index(80));
        assert!(!is_legal_index(81));
        assert!(!is_legal_index(82));
    }
}
