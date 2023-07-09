use macroquad::{
    prelude::{BLACK, GRAY},
    shapes::{draw_line, draw_rectangle},
    text::draw_text_ex,
};

use crate::{
    board::{Board, BoardMode},
    cell_location::CellLocation,
    cell_state::{CellSelection, CellState},
    context::Context,
    fonts::font_types::{CellFont, CellPencilFont},
    index::index_to_xy,
    CELL_COLOR_HIGHLIGHTED, CELL_COLOR_NORMAL, CELL_COLOR_NORMAL_EMPHASIZE,
    CELL_COLOR_PENCIL_EMPHASIZE,
};

pub fn draw_board(context: &Context) {
    draw_board_cells(
        &context.board,
        &context.font_context.initial_font,
        &context.font_context.font,
        &context.font_context.pencil_font,
    );

    draw_cell_lines(context);
    draw_box_lines(context);
}

fn draw_cell(
    cell_state: &CellState,
    cell_location: &CellLocation,
    mode: &BoardMode,
    initial_font: &CellFont,
    font: &CellFont,
    pencil_font: &CellPencilFont,
) {
    let color = if cell_state.selection == CellSelection::Emphasized {
        if *mode == BoardMode::Pencil {
            CELL_COLOR_PENCIL_EMPHASIZE
        } else {
            CELL_COLOR_NORMAL_EMPHASIZE
        }
    } else if cell_state.selection == CellSelection::Highlighted {
        CELL_COLOR_HIGHLIGHTED
    } else {
        CELL_COLOR_NORMAL
    };

    draw_rectangle(
        cell_location.x,
        cell_location.y,
        cell_location.size,
        cell_location.size,
        color,
    );

    if cell_state.has_number() {
        if let Some(n) = cell_state.number {
            draw_text_ex(
                n.to_string().as_str(),
                cell_location.x + font.x_offset,
                cell_location.y + font.y_offset,
                if cell_state.initial {
                    initial_font.params
                } else {
                    font.params
                },
            );
        }
    } else if cell_state.has_pencil() {
        for (i, pencil) in cell_state.pencil.iter().enumerate() {
            if let Some(n) = pencil {
                let (x, y) = index_to_xy(i, 3);

                draw_text_ex(
                    n.to_string().as_str(),
                    cell_location.x + pencil_font.x_offset + (pencil_font.box_size * x as f32),
                    cell_location.y + pencil_font.y_offset + (pencil_font.box_size * y as f32),
                    pencil_font.params,
                );
            }
        }
    }
}

fn draw_board_cells(
    board: &Board,
    initial_font: &CellFont,
    font: &CellFont,
    pencil_font: &CellPencilFont,
) {
    for i in 0..81 {
        draw_cell(
            &board.cell_state[i],
            &board.cell_location[i],
            &board.mode,
            initial_font,
            font,
            pencil_font,
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

        let offset =
            context.game_padding + ((x as f32 * context.board.cell_size) - (line_width / 2.0));
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

        let offset = context.game_padding
            + ((x as f32 * (3.0 * context.board.cell_size)) - (line_width / 2.0));
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
