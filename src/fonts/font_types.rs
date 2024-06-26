use macroquad::prelude::*;
use macroquad::text::TextParams;

use crate::{ICON_DIFFICULTY_1, ICON_PENCIL, ICON_VICTORY_STAR};

use super::font_size::estimate_font_size;

pub struct CellFont {
    pub params: TextParams,
    pub font: Font,
    pub x_offset: f32,
    pub y_offset: f32,
    pub height: f32,
    pub width: f32,
}

impl CellFont {
    pub async fn new(font_path: &str, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        CellFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            x_offset: 0.0,
            y_offset: 0.0,
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = estimate_font_size("9", Some(self.font), cell_size, 1.0);
        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;

        self.x_offset = (cell_size / 2.0) - (self.width / 2.0);
        self.y_offset = (cell_size / 2.0) + (self.height / 2.0);
    }
}

pub struct CellPencilFont {
    pub params: TextParams,
    pub font: Font,
    pub x_offset: f32,
    pub y_offset: f32,
    pub height: f32,
    pub width: f32,
    pub box_size: f32,
}

impl CellPencilFont {
    pub async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        CellPencilFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: BLACK,
            },
            x_offset: 0.0,
            y_offset: 0.0,
            height: measure.height,
            width: measure.width,
            box_size: 0.0,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        let padding = cell_size * 0.1;
        self.box_size = (cell_size - padding) / 3.0;

        self.params.font_size = estimate_font_size("9", Some(self.font), self.box_size, 1.0);
        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;

        self.x_offset = (self.box_size / 2.0) - (self.width / 2.0) + (padding / 2.0);
        self.y_offset = (self.box_size / 2.0) + (self.height / 2.0) + (padding / 2.0);
    }
}

pub struct MenuNumberFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
}

impl MenuNumberFont {
    pub async fn new(font_path: &str, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        MenuNumberFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = estimate_font_size("9", Some(self.font), cell_size, 1.0);

        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;
    }
}

pub struct IconFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
}

impl IconFont {
    pub async fn new(font_path: &str, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text(ICON_PENCIL, Some(font), 48, 0.7);
        IconFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = estimate_font_size(ICON_PENCIL, Some(self.font), cell_size, 0.7);
        let measure = measure_text(ICON_PENCIL, Some(self.font), self.params.font_size, 0.7);
        self.width = measure.width;
        self.height = measure.height;
    }
}

pub struct ModalDifficultyFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
    pub estimate_scale: f32,
}

impl ModalDifficultyFont {
    pub async fn new(font_path: &str, estimate_scale: f32, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text(ICON_DIFFICULTY_1, Some(font), 48, estimate_scale);
        ModalDifficultyFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            height: measure.height,
            width: measure.width,
            estimate_scale,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = estimate_font_size(
            ICON_DIFFICULTY_1,
            Some(self.font),
            cell_size,
            self.estimate_scale,
        );
        let measure = measure_text(
            ICON_DIFFICULTY_1,
            Some(self.font),
            self.params.font_size,
            self.estimate_scale,
        );
        self.width = measure.width;
        self.height = measure.height;
    }
}

pub struct ModalVictoryFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
    pub estimate_scale: f32,
}

impl ModalVictoryFont {
    pub async fn new(font_path: &str, estimate_scale: f32, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text(ICON_VICTORY_STAR, Some(font), 48, 1.0);
        ModalVictoryFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            height: measure.height,
            width: measure.width,
            estimate_scale,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = estimate_font_size(
            ICON_VICTORY_STAR,
            Some(self.font),
            cell_size,
            self.estimate_scale,
        );
        let measure = measure_text(
            ICON_VICTORY_STAR,
            Some(self.font),
            self.params.font_size,
            1.0,
        );
        self.width = measure.width;
        self.height = measure.height;
    }
}
