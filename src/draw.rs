use macroquad::prelude::*;

use crate::{
    board::{Board, BoardMode},
    cell_location::CellLocation,
    cell_state::{CellSelection, CellState},
    context::Context,
    fonts::{CellFont, CellPencilFont},
    CELL_COLOR_EMPHASIZE, CELL_COLOR_HIGHLIGHTED, CELL_COLOR_NORMAL, CELL_COLOR_SELECTED,
    ICON_PENCIL, index::index_to_xy, ICON_PENCIL_SLASH,
};

pub fn draw_cell(
    cell_state: &CellState,
    call_location: &CellLocation,
    initial_font: &CellFont,
    font: &CellFont,
    pencil_font: &CellPencilFont,
) {
    let color = if cell_state.selection == CellSelection::Selected {
        CELL_COLOR_SELECTED
    } else if cell_state.selection == CellSelection::Emphasized {
        CELL_COLOR_EMPHASIZE
    } else if cell_state.selection == CellSelection::Highlighted {
        CELL_COLOR_HIGHLIGHTED
    } else {
        CELL_COLOR_NORMAL
    };

    draw_rectangle(
        call_location.x,
        call_location.y,
        call_location.size,
        call_location.size,
        color,
    );

    if cell_state.has_number() {
        if let Some(n) = cell_state.number {
            draw_text_ex(
                n.to_string().as_str(),
                call_location.x + font.x_offset,
                call_location.y + font.y_offset,
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
                    call_location.x + pencil_font.x_offset + (pencil_font.box_size * x as f32),
                    call_location.y + pencil_font.y_offset + (pencil_font.box_size * y as f32),
                    pencil_font.params,
                );
            }
        }
    }
}

pub fn draw_board(
    board: &Board,
    initial_font: &CellFont,
    font: &CellFont,
    pencil_font: &CellPencilFont,
) {
    for i in 0..81 {
        draw_cell(
            &board.cell_state[i],
            &board.cell_location[i],
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

fn draw_menu(context: &Context) {
    let font_x_offset = context.menu_number_font.width / 2.0;
    let font_y_offset = context.menu_number_font.height + (context.menu_number_font.height / 3.0);

    for (i, number) in context.menu.numbers.iter().enumerate() {
        let digit = i + 1;
        draw_text_ex(
            digit.to_string().as_str(),
            number.x + font_x_offset,
            number.y + font_y_offset,
            context.menu_number_font.params,
        );

        draw_rectangle_lines(number.x, number.y, number.size, number.size, 3.0, BLACK);
    }

    let icon_x_offset = (context.icon_font.width - context.menu.pencil.size).abs() / 2.0;
    let icon_y_offset = (context.icon_font.height - context.menu.pencil.size).abs() / 2.0;

    let icon = match context.board.mode {
        BoardMode::Pencil => ICON_PENCIL,
        BoardMode::Normal => ICON_PENCIL_SLASH,
    };
    
    draw_text_ex(
        icon,
        context.menu.pencil.x + icon_x_offset,
        context.menu.pencil.y + icon_y_offset + context.icon_font.height + (icon_x_offset / 2.0),
        context.icon_font.params,
    );
    draw_rectangle_lines(
        context.menu.pencil.x, 
        context.menu.pencil.y, 
        context.menu.pencil.size,
        context.menu.pencil.size,
        3.0, RED
    );
}

pub fn draw_context(context: &Context) {
    clear_background(WHITE);

    draw_board(
        &context.board,
        &context.initial_font,
        &context.font,
        &context.pencil_font,
    );

    draw_cell_lines(context);
    draw_box_lines(context);

    draw_menu(context);
}
