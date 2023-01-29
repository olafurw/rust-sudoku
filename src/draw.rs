use macroquad::prelude::*;

use crate::{
    cell::Cell, 
    board::Board, cell_font::CellFont, context::Context,
    CELL_COLOR_SELECTED, CELL_COLOR_NORMAL, PADDING, CELL_COLOR_HIGHLIGHTED, CELL_COLOR_EMPHASIZE,
};

pub fn draw_cell(cell: &Cell, text_params: &TextParams, font_x_offset: f32, font_y_offset: f32) {
    let color = if cell.selected { CELL_COLOR_SELECTED } 
                else if cell.emphasize { CELL_COLOR_EMPHASIZE }
                else if cell.highlighted { CELL_COLOR_HIGHLIGHTED } 
                else { CELL_COLOR_NORMAL };

    draw_rectangle(cell.x, cell.y, cell.size, cell.size, color);

    if let Some(n) = cell.number {
        draw_text_ex(
            n.to_string().as_str(), 
            cell.x + font_x_offset, 
            cell.y + font_y_offset, 
            *text_params
        );
    }
}

pub fn draw_board(board: &Board, font: &CellFont) {
    for cell in board.cells.iter() {
        draw_cell(cell, &font.params, font.x_offset, font.y_offset);
    }
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

pub fn draw_context(context: &Context) {
    clear_background(LIGHTGRAY);

    draw_board(&context.board, &context.font);

    draw_cell_lines(context);
    draw_box_lines(context);
}