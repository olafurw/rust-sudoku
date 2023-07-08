use macroquad::prelude::*;

use crate::{
    context::Context, draw_board::draw_board, draw_menu::draw_menu,
    draw_new_game_modal::draw_new_game_modal, draw_victory_modal::draw_victory_modal,
};

pub fn draw_context(context: &Context) {
    clear_background(WHITE);

    draw_board(context);
    draw_menu(context);
    draw_new_game_modal(context);
    draw_victory_modal(context);
}
