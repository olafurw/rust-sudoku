use macroquad::prelude::{BLACK, WHITE};

use crate::{
    CELL_TEXT_COLOR, CELL_TEXT_INITIAL_COLOR, MODAL_DIFFICULTY_ONE, MODAL_DIFFICULTY_THREE,
    MODAL_DIFFICULTY_TWO, MODAL_VICTORY_GOLD, MODAL_VICTORY_RED,
};

use super::fonts::{
    CellFont, CellPencilFont, IconFont, MenuNumberFont, ModalDifficultyFont, ModalVictoryFont,
};

pub struct FontContext {
    pub initial_font: CellFont,
    pub font: CellFont,
    pub icon_font: IconFont,
    pub icon_font_selected: IconFont,
    pub pencil_font: CellPencilFont,
    pub menu_number_font: MenuNumberFont,
    pub menu_number_font_selected: MenuNumberFont,
    pub modal_difficulty_font_1: ModalDifficultyFont,
    pub modal_difficulty_font_2: ModalDifficultyFont,
    pub modal_difficulty_font_3: ModalDifficultyFont,
    pub modal_difficulty_title_font: ModalDifficultyFont,
    pub modal_victory_star_font: ModalVictoryFont,
    pub modal_victory_heart_font: ModalVictoryFont,
}

impl FontContext {
    pub async fn new(font_path: &str, icon_font_path: &str) -> Self {
        FontContext {
            initial_font: CellFont::new(font_path, CELL_TEXT_INITIAL_COLOR).await,
            font: CellFont::new(font_path, CELL_TEXT_COLOR).await,
            icon_font: IconFont::new(icon_font_path, BLACK).await,
            icon_font_selected: IconFont::new(icon_font_path, WHITE).await,
            pencil_font: CellPencilFont::new(font_path).await,
            menu_number_font: MenuNumberFont::new(font_path, BLACK).await,
            menu_number_font_selected: MenuNumberFont::new(font_path, WHITE).await,
            modal_difficulty_font_1: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_ONE,
            )
            .await,
            modal_difficulty_font_2: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_TWO,
            )
            .await,
            modal_difficulty_font_3: ModalDifficultyFont::new(
                icon_font_path,
                1.5,
                MODAL_DIFFICULTY_THREE,
            )
            .await,
            modal_difficulty_title_font: ModalDifficultyFont::new(icon_font_path, 1.0, BLACK).await,
            modal_victory_star_font: ModalVictoryFont::new(icon_font_path, 1.5, MODAL_VICTORY_GOLD)
                .await,
            modal_victory_heart_font: ModalVictoryFont::new(icon_font_path, 1.5, MODAL_VICTORY_RED)
                .await,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.initial_font.update(cell_size);
        self.font.update(cell_size);
        self.pencil_font.update(cell_size);
        self.menu_number_font.update(cell_size);
        self.menu_number_font_selected.update(cell_size);
        self.icon_font.update(cell_size);
        self.icon_font_selected.update(cell_size);
        self.modal_difficulty_font_1.update(cell_size);
        self.modal_difficulty_font_2.update(cell_size);
        self.modal_difficulty_font_3.update(cell_size);
        self.modal_victory_heart_font.update(cell_size);
        self.modal_victory_star_font.update(cell_size);
        self.modal_difficulty_title_font.update(cell_size);
    }
}
