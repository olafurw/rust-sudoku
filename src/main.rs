use macroquad::{prelude::*};

mod cell;

const PADDING: f32 = 10.0;

struct Context {
    font: Font,
    board_size: f32,
    cell_size: f32,
}

impl Context {
    async fn new(font_path: &str) -> Self {
        Context {
            font: load_ttf_font(font_path).await.unwrap(),
            board_size: 0.0,
            cell_size: 0.0,
        }
    }

    fn update(&mut self) {
        self.board_size = screen_width() - (2.0 * PADDING);
        self.cell_size = self.board_size / 9.0;
    }
}

fn update(context: &Context) {

}

fn draw_cell_lines(context: &Context) {
    for x in 1..9 {
        if x % 3 == 0 {
            continue;
        }

        let line_width = context.board_size * 0.0025;
        let line_width = if line_width < 0.5 { 0.5 } else { line_width };

        let offset = PADDING + ((x as f32 * context.cell_size) - (line_width / 2.0));
        draw_line(offset, PADDING, offset, context.board_size + PADDING, line_width, GRAY);
        draw_line(PADDING, offset, context.board_size + PADDING, offset, line_width, GRAY);
    }
}

fn draw_box_lines(context: &Context) {
    for x in 1..3 {
        let line_width = context.board_size * 0.005;
        let line_width = if line_width < 1.0 { 1.0 } else { line_width };

        let offset = PADDING + ((x as f32 * (3.0 * context.cell_size)) - (line_width / 2.0));
        draw_line(offset, PADDING, offset, context.board_size + PADDING, line_width, BLACK);
        draw_line(PADDING, offset, context.board_size + PADDING, offset, line_width, BLACK);
    }
}

fn draw(context: &Context) {
    clear_background(LIGHTGRAY);

    // main squares
    for x in 0..9 {
        for y in 0..9 {
            let width = context.cell_size;
            let x_pos = PADDING + (x as f32 * width);
            let y_pos = PADDING + (y as f32 * width);
            
            draw_rectangle(x_pos, y_pos, width, width, WHITE);
        }
    }

    draw_cell_lines(&context);
    draw_box_lines(&context);
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