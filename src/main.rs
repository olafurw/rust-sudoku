use cell::Cell;
use macroquad::{prelude::*};

mod cell;

const PADDING: f32 = 10.0;
const DIGIT_COUNT: usize = 9;

fn index_to_2d(index: usize) -> (usize, usize) {
    (index % DIGIT_COUNT, index / DIGIT_COUNT)
}

struct Board {
    cells: Vec<Cell>,
    board_size: f32,
    cell_size: f32,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: vec![Cell::new(); 81],
            board_size: 0.0,
            cell_size: 0.0,
        }
    }
    
    fn update(&mut self, board_size: f32) {
        if self.board_size as i32 == board_size as i32 {
            return;
        }

        self.board_size = board_size;
        self.cell_size = self.board_size / 9.0;

        for (i, cell) in self.cells.iter_mut().enumerate() {
            let (x, y) = index_to_2d(i);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }
    }

    fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
}

struct Context {
    font: Font,
    board: Board,
}

impl Context {
    async fn new(font_path: &str) -> Self {
        Context {
            font: load_ttf_font(font_path).await.unwrap(),
            board: Board::new(),
        }
    }

    fn update(&mut self) {
        let board_size = screen_width() - (2.0 * PADDING);
        
        self.board.update(board_size);
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

    context.board.draw();

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