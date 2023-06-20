use macroquad::{
    prelude::WHITE,
    shapes::{draw_circle, draw_poly, draw_rectangle},
    text::draw_text_ex,
    window::{screen_height, screen_width},
};

use crate::{
    context::Context, draw_common::draw_rounded_rectangle, DEBUG_BLUE, ICON_DIFFICULTY_1,
    ICON_DIFFICULTY_2, ICON_DIFFICULTY_3, ICON_DIFFICULTY_NEW, MODAL_BACKGROUND,
};

pub fn draw_new_game_modal(context: &Context) {
    if !context.new_game_modal.show {
        return;
    }

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), MODAL_BACKGROUND);
    draw_rounded_rectangle(
        context.new_game_modal.x,
        context.new_game_modal.y,
        context.new_game_modal.width,
        context.new_game_modal.height,
        20.0,
        WHITE,
    );

    let title_width = context.new_game_modal.width / 3.0;
    let title_x = context.new_game_modal.x + (context.new_game_modal.width / 2.0);
    let title_y =
        context.new_game_modal.y - (context.new_game_modal.height / 3.0) + (title_width / 2.0);
    draw_poly(title_x, title_y, 80, title_width / 2.0, 0., WHITE);

    draw_text_ex(
        ICON_DIFFICULTY_NEW,
        title_x - (context.modal_difficulty_title_font.width / 2.0),
        title_y,
        context.modal_difficulty_title_font.params,
    );

    let extra_y_offset =
        (context.new_game_modal.height / 3.0) + (context.modal_difficulty_font_1.height / 2.0);

    draw_text_ex(
        ICON_DIFFICULTY_1,
        context.new_game_modal.difficulty_1.x,
        context.new_game_modal.difficulty_1.y + extra_y_offset,
        context.modal_difficulty_font_1.params,
    );
    draw_text_ex(
        ICON_DIFFICULTY_2,
        context.new_game_modal.difficulty_2.x,
        context.new_game_modal.difficulty_2.y + extra_y_offset,
        context.modal_difficulty_font_2.params,
    );
    draw_text_ex(
        ICON_DIFFICULTY_3,
        context.new_game_modal.difficulty_3.x,
        context.new_game_modal.difficulty_3.y + extra_y_offset,
        context.modal_difficulty_font_3.params,
    );
}
