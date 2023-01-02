mod cell;
mod board;
mod context;

use context::Context;
use macroquad::{prelude::*};

pub const PADDING: f32 = 10.0;
pub const DIGIT_COUNT: usize = 9;

fn index_to_2d(index: usize) -> (usize, usize) {
    (index % DIGIT_COUNT, index / DIGIT_COUNT)
}

fn update(context: &Context) {

}

fn draw_cell_lines(context: &Context) {
    for x in 1..9 {
        if x % 3 == 0 {
            continue;
        }

        let line_width = context.board.board_size * 0.0025;
        let line_width = if line_width < 0.5 { 0.5 } else { line_width };

        let offset = PADDING + ((x as f32 * context.board.cell_size) - (line_width / 2.0));
        draw_line(offset, PADDING, offset, context.board.board_size + PADDING, line_width, GRAY);
        draw_line(PADDING, offset, context.board.board_size + PADDING, offset, line_width, GRAY);
    }
}

fn draw_box_lines(context: &Context) {
    for x in 1..3 {
        let line_width = context.board.board_size * 0.005;
        let line_width = if line_width < 1.0 { 1.0 } else { line_width };

        let offset = PADDING + ((x as f32 * (3.0 * context.board.cell_size)) - (line_width / 2.0));
        draw_line(offset, PADDING, offset, context.board.board_size + PADDING, line_width, BLACK);
        draw_line(PADDING, offset, context.board.board_size + PADDING, offset, line_width, BLACK);
    }
}

fn draw(context: &Context) {
    clear_background(LIGHTGRAY);

    context.board.draw(&context.params);

    draw_cell_lines(context);
    draw_box_lines(context);
}

#[macroquad::main("Sudoku")]
async fn main() {
    let mut context = Context::new("arial.ttf").await;

    request_new_screen_size(576.0, 1080.0);

    loop {
        context.update();
        update(&context);
        draw(&context);

        next_frame().await
    }
}