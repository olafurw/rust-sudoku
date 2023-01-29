use crate::cell::Cell;
use crate::{PADDING, BOX_INDEXES, ROW_INDEXES, COLUMN_INDEXES};
use crate::context::index_to_xy;

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

        let mut click = (false, None);
        let mut selected_index = 0;

        for (i, cell) in self.cells.iter_mut().enumerate() {
            click = cell.click(x, y);
            if click.0 {
                selected_index = i;
                break;
            }
        }

        // no cell was clicked
        if !click.0 {
            return;
        }

        let mut highlight_list = vec![selected_index];

        // only highlight numbers if the selected cell has a number
        if click.1 != None {
            for (i, cell) in self.cells.iter_mut().enumerate() {
                if cell.number == click.1 {
                    cell.emphasize = true;
                    highlight_list.push(i);
                }
            }
        }

        for index in highlight_list {
            self.highlight_areas(&BOX_INDEXES, index);
            self.highlight_areas(&ROW_INDEXES, index);
            self.highlight_areas(&COLUMN_INDEXES, index);
        }
    }

    fn highlight_areas(&mut self, area: &[[usize; 9]; 9], selected_index: usize) {
        for index_row in area.iter() {
            if index_row.contains(&selected_index) {
                for index in index_row.iter() {
                    self.cells[*index].highlighted = true;
                }
    
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
            let (x, y) = index_to_xy(i);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }
}