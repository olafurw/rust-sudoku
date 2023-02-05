use macroquad::text::TextParams;
use macroquad::{prelude::*};

fn cell_to_font_size(font: &Font, cell_size: f32) -> u16 {
    if cell_size < 1.0 {
        return 1;
    }

    let mut font_size = 1;
    let font = Some(*font);

    // todo: turn into binary search
    for test_size in 1..200 {
        let measurement = measure_text("9", font, test_size, 1.0);
        if measurement.height / cell_size > 0.6 {
            font_size = test_size;
            break;
        }
    }

    font_size
}

pub struct CellFont {
    pub params: TextParams,
    pub font: Font,
    pub x_offset: f32,
    pub y_offset: f32,
    pub height: f32,
    pub width: f32,
}

impl CellFont {
    pub async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        CellFont {
            font,
            params: TextParams { font, font_size: 48, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: BLACK },
            x_offset: 0.0,
            y_offset: 0.0,
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = cell_to_font_size(&self.font, cell_size);
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
            params: TextParams { font, font_size: 48, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: BLACK },
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

        self.params.font_size = cell_to_font_size(&self.font, self.box_size);
        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;

        self.x_offset = (self.box_size / 2.0) - (self.width / 2.0) + (padding / 2.0);
        self.y_offset = (self.box_size / 2.0) + (self.height / 2.0) + (padding / 2.0);
    }
}