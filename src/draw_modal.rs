use macroquad::{
    prelude::WHITE,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

use crate::{context::Context, draw_common::draw_rounded_rectangle, MODAL_BACKGROUND};

pub fn draw_modal(context: &Context) {
    if !context.modal.show {
        return;
    }

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), MODAL_BACKGROUND);
    draw_rounded_rectangle(10.0, 10.0, 100.0, 100.0, 20.0, WHITE);
}
