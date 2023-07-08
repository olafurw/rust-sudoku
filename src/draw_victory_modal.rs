use macroquad::{
    prelude::WHITE,
    shapes::draw_rectangle,
    text::draw_text_ex,
    window::{screen_height, screen_width},
};

use crate::{
    context::Context, draw_common::draw_rounded_rectangle, ICON_VICTORY_HEART, ICON_VICTORY_STAR,
    MODAL_BACKGROUND,
};

pub fn draw_victory_modal(context: &Context) {
    if !context.victory_modal.show {
        return;
    }

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), MODAL_BACKGROUND);
    draw_rounded_rectangle(
        context.victory_modal.x,
        context.victory_modal.y,
        context.victory_modal.width,
        context.victory_modal.height,
        20.0,
        WHITE,
    );

    let extra_y_offset =
        (context.victory_modal.height / 3.0) + (context.modal_victory_star_font.height / 2.0);

    draw_text_ex(
        ICON_VICTORY_STAR,
        context.victory_modal.star_1.x,
        context.victory_modal.star_1.y + extra_y_offset,
        context.modal_victory_star_font.params,
    );
    draw_text_ex(
        ICON_VICTORY_HEART,
        context.victory_modal.heart.x,
        context.victory_modal.heart.y + extra_y_offset,
        context.modal_victory_heart_font.params,
    );
    draw_text_ex(
        ICON_VICTORY_STAR,
        context.victory_modal.star_2.x,
        context.victory_modal.star_2.y + extra_y_offset,
        context.modal_victory_star_font.params,
    );
}
