use macroquad::prelude::*;

use crate::{
    board::Board,
    cell::Cell,
    cell_font::{CellFont, CellPencilFont},
    context::{index_to_xy, Context},
    CELL_COLOR_EMPHASIZE, CELL_COLOR_HIGHLIGHTED, CELL_COLOR_NORMAL, CELL_COLOR_SELECTED, PADDING,
};

pub fn draw_cell(cell: &Cell, font: &CellFont, pencil_font: &CellPencilFont) {
    let color = if cell.selected {
        CELL_COLOR_SELECTED
    } else if cell.emphasize {
        CELL_COLOR_EMPHASIZE
    } else if cell.highlighted {
        CELL_COLOR_HIGHLIGHTED
    } else {
        CELL_COLOR_NORMAL
    };

    draw_rectangle(cell.x, cell.y, cell.size, cell.size, color);

    if cell.has_number() {
        if let Some(n) = cell.number {
            draw_text_ex(
                n.to_string().as_str(),
                cell.x + font.x_offset,
                cell.y + font.y_offset,
                font.params,
            );
        }
    } else if cell.has_pencil() {
        for (i, pencil) in cell.pencil.iter().enumerate() {
            if let Some(n) = pencil {
                let (x, y) = index_to_xy(i, 3);

                draw_text_ex(
                    n.to_string().as_str(),
                    cell.x + pencil_font.x_offset + (pencil_font.box_size * x as f32),
                    cell.y + pencil_font.y_offset + (pencil_font.box_size * y as f32),
                    pencil_font.params,
                );
            }
        }
    }
}

pub fn draw_board(board: &Board, font: &CellFont, pencil_font: &CellPencilFont) {
    for cell in board.cells.iter() {
        draw_cell(cell, font, pencil_font);
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
        draw_line(
            offset,
            PADDING,
            offset,
            context.board.board_size + PADDING,
            line_width,
            GRAY,
        );
        draw_line(
            PADDING,
            offset,
            context.board.board_size + PADDING,
            offset,
            line_width,
            GRAY,
        );
    }
}

fn draw_box_lines(context: &Context) {
    for x in 0..4 {
        let line_width = context.board.board_size * 0.005;
        let line_width = if line_width < 1.0 { 1.0 } else { line_width };

        let offset = PADDING + ((x as f32 * (3.0 * context.board.cell_size)) - (line_width / 2.0));
        draw_line(
            offset,
            PADDING,
            offset,
            context.board.board_size + PADDING,
            line_width,
            BLACK,
        );
        draw_line(
            PADDING - line_width,
            offset,
            context.board.board_size + PADDING,
            offset,
            line_width,
            BLACK,
        );
    }
}

pub fn draw_context(context: &Context) {
    clear_background(WHITE);

    draw_board(&context.board, &context.font, &context.pencil_font);

    draw_cell_lines(context);
    draw_box_lines(context);
}
