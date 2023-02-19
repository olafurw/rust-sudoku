use macroquad::prelude::*;

use crate::{
    board::Board,
    cell_state::{CellState, CellSelection},
    fonts::{CellFont, CellPencilFont},
    context::{index_to_xy, Context},
    CELL_COLOR_EMPHASIZE, CELL_COLOR_HIGHLIGHTED, CELL_COLOR_NORMAL, CELL_COLOR_SELECTED, 
    cell_location::CellLocation,
};

pub fn draw_cell(cell_state: &CellState, call_location: &CellLocation, initial_font: &CellFont, font: &CellFont, pencil_font: &CellPencilFont) {
    let color = if cell_state.selection == CellSelection::Selected {
        CELL_COLOR_SELECTED
    } else if cell_state.selection == CellSelection::Emphasized {
        CELL_COLOR_EMPHASIZE
    } else if cell_state.selection == CellSelection::Highlighted {
        CELL_COLOR_HIGHLIGHTED
    } else {
        CELL_COLOR_NORMAL
    };

    draw_rectangle(call_location.x, call_location.y, call_location.size, call_location.size, color);

    if cell_state.has_number() {
        if let Some(n) = cell_state.number {
            draw_text_ex(
                n.to_string().as_str(),
                call_location.x + font.x_offset,
                call_location.y + font.y_offset,
                if cell_state.initial { initial_font.params } else { font.params },
            );
        }
    } else if cell_state.has_pencil() {
        for (i, pencil) in cell_state.pencil.iter().enumerate() {
            if let Some(n) = pencil {
                let (x, y) = index_to_xy(i, 3);

                draw_text_ex(
                    n.to_string().as_str(),
                    call_location.x + pencil_font.x_offset + (pencil_font.box_size * x as f32),
                    call_location.y + pencil_font.y_offset + (pencil_font.box_size * y as f32),
                    pencil_font.params,
                );
            }
        }
    }
}

pub fn draw_board(board: &Board, initial_font: &CellFont, font: &CellFont, pencil_font: &CellPencilFont) {
    for i in 0..81 {
        draw_cell(
            &board.cell_state[i], 
            &board.cell_location[i], 
            initial_font, font, pencil_font
        );
    }
}

fn draw_cell_lines(context: &Context) {
    for x in 1..9 {
        if x % 3 == 0 {
            continue;
        }

        let line_width = context.board.board_size * 0.0025;
        let line_width = if line_width < 0.5 { 0.5 } else { line_width };

        let offset = context.game_padding + ((x as f32 * context.board.cell_size) - (line_width / 2.0));
        draw_line(
            offset,
            context.game_padding,
            offset,
            context.board.board_size + context.game_padding,
            line_width,
            GRAY,
        );
        draw_line(
            context.game_padding,
            offset,
            context.board.board_size + context.game_padding,
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

        let offset = context.game_padding + ((x as f32 * (3.0 * context.board.cell_size)) - (line_width / 2.0));
        draw_line(
            offset,
            context.game_padding,
            offset,
            context.board.board_size + context.game_padding,
            line_width,
            BLACK,
        );
        draw_line(
            context.game_padding - line_width,
            offset,
            context.board.board_size + context.game_padding,
            offset,
            line_width,
            BLACK,
        );
    }
}

fn draw_menu(context: &Context) {
    let start_y = context.game_square as f32 + context.menu_number_font.height + (context.menu_number_font.height / 2.0);
    let menu_width = context.board_size;

    for i in 1..=9 {
        draw_text_ex(
            i.to_string().as_str(),
            context.game_padding + (context.menu_number_font.width / 2.0) + ((i - 1) as f32 * (menu_width / 9.0)),
            start_y,
            context.menu_number_font.params
        );
    }
}

pub fn draw_context(context: &Context) {
    clear_background(WHITE);

    draw_board(&context.board, &context.initial_font, &context.font, &context.pencil_font);

    draw_cell_lines(context);
    draw_box_lines(context);

    draw_menu(context);
}
