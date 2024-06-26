use crate::{is_legal_number, menu_item::MenuItem};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MenuActions {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Pencil = 10,
    Undo = 11,
    New = 12,
    Delete = 13,
}

fn menu_action_from_u8(value: u8) -> Option<MenuActions> {
    match value {
        1 => Some(MenuActions::One),
        2 => Some(MenuActions::Two),
        3 => Some(MenuActions::Three),
        4 => Some(MenuActions::Four),
        5 => Some(MenuActions::Five),
        6 => Some(MenuActions::Six),
        7 => Some(MenuActions::Seven),
        8 => Some(MenuActions::Eight),
        9 => Some(MenuActions::Nine),
        _ => None,
    }
}

pub fn is_menu_action_number(action: MenuActions) -> bool {
    let number = action as u8;
    is_legal_number(number)
}

pub struct Menu {
    pub board_size: f32,
    pub item_size: f32,
    pub menu_start_x: f32,
    pub menu_start_y: f32,
    pub menu_height: f32,
    pub menu_width: f32,
    pub game_padding: f32,
    pub portrait: bool,
    pub numbers: [MenuItem; 9],
    pub pencil: MenuItem,
    pub undo: MenuItem,
    pub new: MenuItem,
    pub delete: MenuItem,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            board_size: 0.0,
            item_size: 0.0,
            menu_start_x: 0.0,
            menu_start_y: 0.0,
            menu_height: 0.0,
            menu_width: 0.0,
            game_padding: 0.0,
            portrait: true,
            numbers: [Default::default(); 9],
            pencil: Default::default(),
            undo: Default::default(),
            new: Default::default(),
            delete: Default::default(),
        }
    }

    fn update_portrait(&mut self) {
        self.menu_start_x = self.game_padding;
        let mut start_x = self.menu_start_x;

        self.menu_start_y = self.board_size + (2.0 * self.game_padding);

        self.menu_height = (self.board_size + (2.0 * self.game_padding)) / 3.0;
        self.menu_width = self.board_size;

        for number in self.numbers.iter_mut() {
            number.update(start_x, self.menu_start_y, self.item_size);
            start_x += self.item_size;
        }

        let second_row_y = self.item_size + (self.item_size / 2.0);
        self.undo.update(
            self.game_padding,
            self.menu_start_y + second_row_y,
            self.item_size,
        );

        self.pencil.update(
            self.game_padding + self.item_size,
            self.menu_start_y + second_row_y,
            self.item_size,
        );

        self.delete.update(
            self.game_padding + (2.0 * self.item_size),
            self.menu_start_y + second_row_y,
            self.item_size,
        );

        self.new.update(
            self.game_padding + (8.0 * self.item_size),
            self.menu_start_y + second_row_y,
            self.item_size,
        );
    }

    fn update_landscape(&mut self) {
        self.menu_start_x = self.board_size + (2.0 * self.game_padding);

        self.menu_start_y = self.game_padding;
        let mut start_y = self.menu_start_y;

        self.menu_width = (self.board_size + (2.0 * self.game_padding)) / 3.0;
        self.menu_height = self.board_size;

        for number in self.numbers.iter_mut() {
            number.update(self.menu_start_x, start_y, self.item_size);
            start_y += self.item_size;
        }

        let second_row_x = self.item_size + (self.item_size / 2.0);
        self.undo.update(
            self.menu_start_x + second_row_x,
            self.game_padding,
            self.item_size,
        );

        self.pencil.update(
            self.menu_start_x + second_row_x,
            self.game_padding + self.item_size,
            self.item_size,
        );

        self.delete.update(
            self.menu_start_x + second_row_x,
            self.game_padding + (2.0 * self.item_size),
            self.item_size,
        );

        self.new.update(
            self.menu_start_x + second_row_x,
            self.game_padding + (8.0 * self.item_size),
            self.item_size,
        );
    }

    pub fn update(&mut self, board_size: f32, game_padding: f32, portrait: bool) {
        self.board_size = board_size;
        self.item_size = board_size / 9.0;
        self.game_padding = game_padding;
        self.portrait = portrait;

        if self.portrait {
            self.update_portrait();
        } else {
            self.update_landscape();
        }
    }

    pub fn click(&self, x: f32, y: f32) -> Option<MenuActions> {
        for (i, number) in self.numbers.iter().enumerate() {
            if number.click(x, y) {
                return menu_action_from_u8((i + 1) as u8);
            }
        }

        if self.pencil.click(x, y) {
            return Some(MenuActions::Pencil);
        }

        if self.delete.click(x, y) {
            return Some(MenuActions::Delete);
        }

        if self.undo.click(x, y) {
            return Some(MenuActions::Undo);
        }

        if self.new.click(x, y) {
            return Some(MenuActions::New);
        }

        None
    }
}
