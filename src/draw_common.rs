use macroquad::{
    prelude::{vec2, Color},
    shapes::{draw_rectangle, draw_triangle},
};

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

pub fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
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
