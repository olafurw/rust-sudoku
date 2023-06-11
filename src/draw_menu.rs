use macroquad::{
    prelude::{vec2, Color, BLUE},
    shapes::{draw_rectangle, draw_triangle},
    text::draw_text_ex,
};

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

fn draw_quarter_circle(center_x: f32, center_y: f32, radius: f32, angle: f32, color: Color) {
    const NUM_TRIANGLES: u32 = 10;
    const ANGLE_STEP: f32 = std::f32::consts::FRAC_PI_2 / NUM_TRIANGLES as f32;

    for i in 0..NUM_TRIANGLES {
        let start_angle = angle + ANGLE_STEP * i as f32;
        let end_angle = angle + ANGLE_STEP * (i + 1) as f32;

        let start_x = center_x + radius * start_angle.cos();
        let start_y = center_y + radius * start_angle.sin();
        let end_x = center_x + radius * end_angle.cos();
        let end_y = center_y + radius * end_angle.sin();

        draw_triangle(
            vec2(center_x, center_y),
            vec2(start_x, start_y),
            vec2(end_x, end_y),
            color,
        );
    }
}

fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    const DEG180: f32 = std::f32::consts::PI;
    const DEG90: f32 = std::f32::consts::FRAC_PI_2;
    const DEG270: f32 = DEG180 + DEG90;

    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
    draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);

    draw_quarter_circle(x + radius, y + radius, radius, DEG180, color);
    draw_quarter_circle(x + width - radius, y + radius, radius, DEG270, color);
    draw_quarter_circle(x + width - radius, y + height - radius, radius, 0.0, color);
    draw_quarter_circle(x + radius, y + height - radius, radius, DEG90, color);
}

fn draw_menu_numbers(context: &Context) {
    let border_offset = context.board.board_size * 0.005;
    let font_x_offset = (context.menu_number_font.width / 2.0) + border_offset;
    let font_y_offset = context.menu_number_font.height + (context.menu_number_font.height / 3.0);

    let selected_number = if context.selected_number.is_some() {
        context.selected_number.unwrap()
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
                BLUE,
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

    /*draw_rectangle(
        context.menu.menu_start_x,
        context.menu.menu_start_y,
        context.menu.menu_width,
        context.menu.menu_height,
        DEBUG_BLUE,
    );*/
}
