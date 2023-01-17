// TODO
//
// create draw trait and seperate them into their own file
// board, cell_font, cell should only be state update stuff, not drawing
// board should also respect the height of the screen to allow for input buttons
// clicking should select cell and highlight areas
// selecting a cell with a value should highlight same values

mod board;
mod cell;
mod context;
mod cell_font;

use context::Context;
use macroquad::prelude::*;

pub const PADDING: f32 = 10.0;
pub const DIGIT_COUNT: usize = 9;
pub const CELL_COLOR_NORMAL: Color = WHITE;
pub const CELL_COLOR_SELECTED: Color = Color::new(0.70, 0.85, 1.00, 1.00);

#[macroquad::main("Sudoku")]
async fn main() {
    let mut context = Context::new("arial.ttf").await;

    request_new_screen_size(576.0, 1080.0);

    loop {
        context.update();
        context.draw();

        next_frame().await
    }
}