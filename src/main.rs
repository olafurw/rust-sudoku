use macroquad::{prelude::*};

mod cell;
use cell::Cell;

#[macroquad::main("Sudoku")]
async fn main() {
    let texture: Texture2D = load_texture("image.png").await.unwrap();
    let c = Cell::new();

    request_new_screen_size(720.0, 1080.0);

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            info!("{}, {}", x, y);
        }

        draw_texture(texture, 0.0, 0.0, WHITE);
        c.draw();

        next_frame().await
    }
}