use macroquad::prelude::*;

use crate::{
    context::Context, draw_board::draw_board, draw_menu::draw_menu, draw_modal::draw_modal,
};

pub fn draw_context(context: &Context) {
    clear_background(WHITE);

    draw_board(context);
    draw_menu(context);
    draw_modal(context);
}
