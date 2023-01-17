#[derive(Clone)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub number: Option<u32>,
    pub selected: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            x: 0.0, y: 0.0, size: 0.0, number: Some(1), selected: false
        }
    }

    pub fn clear(&mut self) {
        self.selected = false;
    }

    pub fn click(&mut self, x: f32, y: f32) -> bool {
        if x >= self.x && x <= self.x + self.size
        && y >= self.y && y <= self.y + self.size {
            self.selected = true;
        }

        self.selected
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }
}