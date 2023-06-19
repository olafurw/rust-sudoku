use macroquad::text::draw_text_ex;

use crate::{
    board::BoardMode, context::Context, draw_common::draw_rounded_rectangle, ICON_NEW, ICON_PENCIL,
    ICON_PENCIL_SLASH, ICON_UNDO, MENU_NUMBER_BACKGROUND_NORMAL, MENU_NUMBER_BACKGROUND_PENCIL,
};

fn draw_menu_pencil(context: &Context, icon_x_offset: f32, icon_y_offset: f32) {
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
}

fn draw_menu_undo(context: &Context, icon_x_offset: f32, icon_y_offset: f32) {
    draw_text_ex(
        ICON_UNDO,
        context.menu.undo.x + icon_x_offset,
        context.menu.undo.y + icon_y_offset + context.icon_font.height + (icon_x_offset / 2.0),
        context.icon_font.params,
    );
}

fn draw_menu_new(context: &Context, icon_x_offset: f32, icon_y_offset: f32) {
    draw_text_ex(
        ICON_NEW,
        context.menu.new.x + icon_x_offset,
        context.menu.new.y + icon_y_offset + context.icon_font.height + (icon_x_offset / 2.0),
        context.icon_font.params,
    );
}

fn draw_menu_numbers(context: &Context) {
    let border_offset = context.board.board_size * 0.005;
    let font_x_offset = (context.menu_number_font.width / 2.0) + border_offset;
    let font_y_offset = context.menu_number_font.height + (context.menu_number_font.height / 3.0);

    let selected_number = if context.board.selected_number.is_some() {
        context.board.selected_number.unwrap()
    } else {
        0
    };

    for (i, number) in context.menu.numbers.iter().enumerate() {
        if context.board.number_count[i] == 9 {
            continue;
        }

        let digit = i + 1;
        if digit == selected_number as usize {
            draw_rounded_rectangle(
                number.x,
                number.y,
                context.menu.item_size,
                context.menu.item_size,
                20.0,
                if context.board.mode == BoardMode::Normal {
                    MENU_NUMBER_BACKGROUND_NORMAL
                } else {
                    MENU_NUMBER_BACKGROUND_PENCIL
                },
            );
        }

        draw_text_ex(
            digit.to_string().as_str(),
            number.x + font_x_offset,
            number.y + font_y_offset,
            if digit == selected_number as usize {
                context.menu_number_font_selected.params
            } else {
                context.menu_number_font.params
            },
        );
    }
}

pub fn draw_menu(context: &Context) {
    draw_menu_numbers(context);

    let icon_x_offset = (context.icon_font.width - context.menu.pencil.size).abs() / 2.0;
    let icon_y_offset = (context.icon_font.height - context.menu.pencil.size).abs() / 2.0;

    draw_menu_pencil(context, icon_x_offset, icon_y_offset);
    draw_menu_undo(context, icon_x_offset, icon_y_offset);
    draw_menu_new(context, icon_x_offset, icon_y_offset);
}
