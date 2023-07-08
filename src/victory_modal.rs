use crate::menu_item::MenuItem;

pub struct VictoryModal {
    pub show: bool,
    pub x: f32,
    pub y: f32,
    pub game_square: f32,
    pub font_height: f32,
    pub font_width: f32,
    pub width: f32,
    pub height: f32,
    pub heart: MenuItem,
    pub star_1: MenuItem,
    pub star_2: MenuItem,
}

impl Default for VictoryModal {
    fn default() -> Self {
        Self::new()
    }
}

impl VictoryModal {
    pub fn new() -> Self {
        VictoryModal {
            show: false,
            x: 0.0,
            y: 0.0,
            game_square: 0.0,
            font_height: 0.0,
            font_width: 0.0,
            width: 0.0,
            height: 0.0,
            heart: Default::default(),
            star_1: Default::default(),
            star_2: Default::default(),
        }
    }

    pub fn show(&mut self) {
        self.show = true;

        self.update_victory();
    }

    pub fn hide(&mut self) {
        self.show = false;
    }

    pub fn update(&mut self, game_square: f32, font_width: f32, font_height: f32) {
        self.game_square = game_square;
        self.font_width = font_width;
        self.font_height = font_height;

        if self.show {
            self.update_victory();
        }
    }

    pub fn update_victory(&mut self) {
        let cell_width = self.game_square / 9.0;
        self.width = cell_width * 4.0;
        self.x = self.game_square / 2.0 - (self.width / 2.0);
        self.height = cell_width * 1.5;
        self.y = self.game_square / 2.0 - (self.height / 2.0);

        let button_y = self.y + self.height / 2.0 - self.font_height / 2.0;

        let button_2_x = (self.game_square / 2.0) - (self.font_width / 2.0);
        self.heart.update(button_2_x, button_y, cell_width);

        let button_1_x = button_2_x - (cell_width * 1.25);
        self.star_1.update(button_1_x, button_y, cell_width);

        let button_3_x = button_2_x + (cell_width * 1.25);
        self.star_2.update(button_3_x, button_y, cell_width);
    }

    pub fn click_outside(&mut self, x: f32, y: f32) -> bool {
        x < self.x || x > self.x + self.width || y < self.y || y > self.y + self.height
    }
}
