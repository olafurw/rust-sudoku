use crate::cell::Cell;
use crate::cell_font::CellFont;
use crate::PADDING;
use crate::context::index_to_2d;

pub struct Board {
    pub cells: Vec<Cell>,
    pub board_size: f32,
    pub cell_size: f32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::new(); 81],
            board_size: 0.0,
            cell_size: 0.0,
        }
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
    }

    pub fn click(&mut self, x: f32, y: f32) {
        self.clear();
        
        for cell in self.cells.iter_mut() {
            if cell.click(x, y) {
                break;
            }
        }
    }
    
    pub fn update(&mut self, board_size: f32) -> bool {
        if self.board_size as i32 == board_size as i32 {
            return false;
        }

        self.board_size = board_size;
        self.cell_size = self.board_size / 9.0;

        for (i, cell) in self.cells.iter_mut().enumerate() {
            let (x, y) = index_to_2d(i);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }

    pub fn draw(&self, font: &CellFont) {
        for cell in self.cells.iter() {
            cell.draw(&font.params, font.x_offset, font.y_offset);
        }
    }
}