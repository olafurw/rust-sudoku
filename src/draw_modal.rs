use macroquad::{
    prelude::WHITE,
    shapes::draw_rectangle,
    text::draw_text_ex,
    window::{screen_height, screen_width},
};

use crate::{
    context::Context, draw_common::draw_rounded_rectangle, DEBUG_BLUE, ICON_DIFFICULTY_1,
    ICON_DIFFICULTY_2, ICON_DIFFICULTY_3, MODAL_BACKGROUND,
};

pub fn draw_modal(context: &Context) {
    if !context.modal.show {
        return;
    }

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), MODAL_BACKGROUND);
    draw_rounded_rectangle(
        context.modal.x,
        context.modal.y,
        context.modal.width,
        context.modal.height,
        20.0,
        WHITE,
    );

    let icon_x_offset =
        (context.modal_difficulty_font_1.width - context.menu.pencil.size).abs() / 2.0;
    let icon_y_offset =
        (context.modal_difficulty_font_1.height - context.menu.pencil.size).abs() / 2.0;
    let extra_y_offset = context.modal_difficulty_font_1.height + (icon_x_offset / 2.0);

    draw_text_ex(
        ICON_DIFFICULTY_1,
        context.modal.difficulty_1.x + icon_x_offset,
        context.modal.difficulty_1.y + icon_y_offset + extra_y_offset,
        context.modal_difficulty_font_1.params,
    );
    draw_text_ex(
        ICON_DIFFICULTY_2,
        context.modal.difficulty_2.x + icon_x_offset,
        context.modal.difficulty_2.y + icon_y_offset + extra_y_offset,
        context.modal_difficulty_font_2.params,
    );
    draw_text_ex(
        ICON_DIFFICULTY_3,
        context.modal.difficulty_3.x + icon_x_offset,
        context.modal.difficulty_3.y + icon_y_offset + extra_y_offset,
        context.modal_difficulty_font_3.params,
    );

    draw_rectangle(
        context.modal.difficulty_2.x,
        context.modal.difficulty_2.y,
        context.modal.difficulty_2.size,
        context.modal.difficulty_2.size,
        DEBUG_BLUE,
    );
}
