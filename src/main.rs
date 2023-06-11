// TODO
//
// make the menu numbers a little prettier
// add text under the menu
// hightlight mode switcher
// move the menu to the other side in landscape
// when shrinking due to space missing, center the game board
// add WAY more tests
// clicking a note again with the same number should remove it

mod board;
mod cell_location;
mod cell_state;
mod context;
mod draw;
mod draw_board;
mod draw_menu;
mod font_size;
mod fonts;
mod generate;
mod index;
mod menu;
mod menu_item;

use context::Context;
use draw::draw_context;
use egui_macroquad::egui;
use macroquad::prelude::*;

pub const DIGIT_COUNT: usize = 9;
pub const CELL_COLOR_NORMAL: Color = color_u8!(255, 255, 255, 255);
pub const CELL_COLOR_SELECTED: Color = color_u8!(255, 208, 177, 255);
pub const CELL_COLOR_EMPHASIZE: Color = color_u8!(178, 216, 255, 255);
pub const CELL_COLOR_HIGHLIGHTED: Color = color_u8!(219, 219, 219, 255);

pub const CELL_TEXT_COLOR: Color = color_u8!(44, 96, 142, 255);
pub const CELL_TEXT_INITIAL_COLOR: Color = color_u8!(0, 0, 0, 255);

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

pub fn is_legal_number(number: u8) -> bool {
    (1..=9).contains(&number)
}

#[macroquad::main("Sudoku")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let mut context = Context::new("liberation-sans.ttf", "material-font.ttf").await;

    request_new_screen_size(800.0, 1080.0);

    loop {
        context.update();
        draw_context(&context);

        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_visuals(egui::Visuals::light());
            egui::Window::new("debug").show(egui_ctx, |ui| {
                ui.label(format!("selected: {:?}", context.selected_number));
            });
        });

        egui_macroquad::draw();

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use crate::is_legal_number;

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
    }
}
