mod board;
mod cell;
mod context;
mod cell_font;

use context::Context;
use macroquad::prelude::*;

pub const PADDING: f32 = 10.0;
pub const DIGIT_COUNT: usize = 9;

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