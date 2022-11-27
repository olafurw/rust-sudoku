mod cell;
mod board;

use board::Board;
use macroquad::{prelude::*};

pub const PADDING: f32 = 10.0;
pub const DIGIT_COUNT: usize = 9;

pub fn index_to_2d(index: usize) -> (usize, usize) {
    (index % DIGIT_COUNT, index / DIGIT_COUNT)
}

pub fn cell_font(font: &Font, cell_size: f32) -> u16 {
    if cell_size < 1.0 {
        return 1;
    }

    let mut font_size = 1;

    // todo: turn into binary search
    for test_size in 1..200 {
        let measurement = measure_text("9", Some(*font), test_size, 1.0);
        if measurement.height / cell_size > 0.6 {
            font_size = test_size;
            break;
        }
    }

    font_size
}

struct Context {
    params: TextParams,
    font_height: f32,
    font_width: f32,
    board: Board,
}

impl Context {
    async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        Context {
            params: TextParams { font, font_size: 48, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: BLACK },
            font_height: measure.height,
            font_width: measure.width,
            board: Board::new(),
        }
    }

    fn update(&mut self) {
        let board_size = screen_width() - (2.0 * PADDING);
        if !self.board.update(board_size) {
            return;
        }

        self.params.font_size = cell_font(&self.params.font, self.board.cell_size);
        
        let measure = measure_text("9", Some(self.params.font), self.params.font_size, 1.0);
        self.board.update_font_offset(measure.height, measure.width);
    }
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