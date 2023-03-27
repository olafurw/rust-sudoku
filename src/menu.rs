use crate::menu_item::MenuItem;

pub struct Menu {
    pub board_size: f32,
    pub game_padding: f32,
    pub portrait: bool,
    pub numbers: [MenuItem; 9],
    pub pencil_mode: MenuItem,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            board_size: 0.0,
            game_padding: 0.0,
            portrait: true,
            numbers: [Default::default(); 9],
            pencil_mode: Default::default(),
        }
    }

    pub fn update(&mut self, board_size: f32, game_padding: f32, portrait: bool) {
        self.board_size = board_size;
        self.game_padding = game_padding;
        self.portrait = portrait;

        let number_box = self.board_size / 9.0;
        let mut start_x = self.game_padding;
        let start_y = self.board_size + (2.0 * self.game_padding);

        for number in self.numbers.iter_mut() {
            number.update(start_x, start_y, number_box);
            start_x += number_box;
        }

        self.pencil_mode.update(100.0, 100.0, number_box);
    }

    pub fn click(&self, x: f32, y: f32) -> Option<u8> {
        for (i, number) in self.numbers.iter().enumerate() {
            if number.click(x, y) {
                return Some((i + 1) as u8);
            }
        }

        None
    }
}
