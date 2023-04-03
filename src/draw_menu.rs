use macroquad::text::draw_text_ex;

use crate::{board::BoardMode, context::Context, ICON_PENCIL, ICON_PENCIL_SLASH, ICON_UNDO};

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

fn draw_menu_numbers(context: &Context) {
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
    }
}

pub fn draw_menu(context: &Context) {
    draw_menu_numbers(context);

    let icon_x_offset = (context.icon_font.width - context.menu.pencil.size).abs() / 2.0;
    let icon_y_offset = (context.icon_font.height - context.menu.pencil.size).abs() / 2.0;

    draw_menu_pencil(context, icon_x_offset, icon_y_offset);
    draw_menu_undo(context, icon_x_offset, icon_y_offset);
}
