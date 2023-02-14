// TODO
//
// actual undo, with action stack
// more unit tests

mod board;
mod cell;
mod cell_font;
mod context;
mod draw;

use context::Context;
use draw::draw_context;
use macroquad::{prelude::*, ui::{Skin, root_ui}};

pub const PADDING: f32 = 10.0;
pub const DIGIT_COUNT: usize = 9;
pub const CELL_COLOR_NORMAL: Color = color_u8!(255, 255, 255, 255);
pub const CELL_COLOR_SELECTED: Color = color_u8!(255, 208, 177, 255);
pub const CELL_COLOR_EMPHASIZE: Color = color_u8!(178, 216, 255, 255);
pub const CELL_COLOR_HIGHLIGHTED: Color = color_u8!(209, 209, 209, 255);

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

#[macroquad::main("Sudoku")]
async fn main() {
    let skin = {
        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../button_normal.png"),
                None,
            ))
            .background_margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("../button_normal_hover.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("../button_normal_clicked.png"),
                None,
            ))
            .font(include_bytes!("../arial.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(255, 255, 255, 255))
            .text_color_hovered(Color::from_rgba(255, 255, 255, 255))
            .text_color_clicked(Color::from_rgba(255, 255, 255, 255))
            .font_size(40)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../window_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .build();
        
        Skin {
            button_style,
            window_style,
            ..root_ui().default_skin()
        }
    };

    let mut context = Context::new("arial.ttf", skin.clone()).await;

    request_new_screen_size(576.0, 1080.0);

    loop {
        context.update();
        draw_context(&context);

        next_frame().await
    }
}
