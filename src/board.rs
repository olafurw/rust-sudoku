use std::collections::HashSet;
use std::mem::size_of;

use crate::cell_location::CellLocation;
use crate::cell_state::{CellState, CellSelection};
use crate::context::index_to_xy;
use crate::{BOX_INDEXES, COLUMN_INDEXES, DIGIT_COUNT, PADDING, ROW_INDEXES};

pub struct Board {
    pub cell_state: [CellState; 81],
    pub cell_location: [CellLocation; 81],
    pub board_size: f32,
    pub cell_size: f32,
    pub selected_index: Option<usize>,
    pub selected_number: Option<u8>,
    pub undo_number: Option<Option<u8>>,
}

impl Board {
    pub fn new() -> Self {
        println!("{}", size_of::<CellState>());
        Board {
            cell_state: [Default::default(); 81],
            cell_location: [Default::default(); 81],
            board_size: 0.0,
            cell_size: 0.0,
            selected_index: None,
            selected_number: None,
            undo_number: None,
        }
    }

    pub fn clear_cell_selection(&mut self) {
        for cell in self.cell_state.iter_mut() {
            cell.clear_selection();
        }
    }

    pub fn clear_number(&mut self) {
        if self.selected_index.is_none() {
            return;
        }

        self.cell_state[self.selected_index.unwrap()].clear_number();
    }

    fn is_number_initial(&self) -> bool {
        if self.selected_index.is_none() {
            return false;
        }

        self.cell_state[self.selected_index.unwrap()].initial
    }

    pub fn number(&mut self, number: u8) {
        if self.selected_index.is_none() {
            return;
        }

        if self.is_number_initial() {
            self.handle_if_pencil(number);
            return;
        }

        self.selected_number = Some(number);
        self.handle_if_pencil(number);
        self.handle_if_insert(number);

        if self.is_valid() {
            self.highlight();
            self.clear_pencil(number);
            return;
        }

        // ok we're not valid, let's undo the work we did
        if self.undo_number.is_some() {
            let cell = &mut self.cell_state[self.selected_index.unwrap()];
            cell.number = self.undo_number.unwrap();
            self.undo_number = None;
        }
    }

    fn handle_if_pencil(&mut self, number: u8) {
        let cell = &self.cell_state[self.selected_index.unwrap()];
        if !cell.is_number(number) {
            return;
        }

        self.highlight();
        self.pencil_unhighlighted(number);
    }

    fn handle_if_insert(&mut self, number: u8) {
        let cell = &mut self.cell_state[self.selected_index.unwrap()];
        if cell.is_number(number) {
            return;
        }

        self.undo_number = Some(cell.number);
        cell.set_number(number);
    }

    pub fn click(&mut self, x: f32, y: f32) {
        let mut clicked = false;
        self.selected_index = None;

        // perform a click on each cell to see which one
        // gets selected
        for i in 0..81 {
            let loc = &self.cell_location[i];
            clicked = loc.click(x, y);
            if clicked {
                let cell = &self.cell_state[i];
                self.selected_index = Some(i);
                self.selected_number = cell.number;
                break;
            }
        }

        // no cell was clicked
        if !clicked {
            self.clear_cell_selection();
            return;
        }

        self.highlight();
    }

    fn pencil_unhighlighted(&mut self, number: u8) {
        for cell in self.cell_state.iter_mut() {
            if cell.has_number() || cell.selection != CellSelection::None {
                continue;
            }

            cell.set_pencil(number);
        }
    }

    fn clear_pencil(&mut self, number: u8) {
        for cell in self.cell_state.iter_mut() {
            if cell.has_number() || cell.selection == CellSelection::Selected || cell.selection == CellSelection::Emphasized {
                continue;
            }

            if cell.selection == CellSelection::Highlighted {
                cell.remove_pencil(number);
            }
        }
    }

    fn is_valid(&self) -> bool {
        for range in BOX_INDEXES {
            if !self.is_range_valid(range) {
                return false;
            }
        }

        for range in ROW_INDEXES {
            if !self.is_range_valid(range) {
                return false;
            }
        }

        for range in COLUMN_INDEXES {
            if !self.is_range_valid(range) {
                return false;
            }
        }

        true
    }

    fn is_range_valid(&self, range: &[usize; 9]) -> bool {
        let mut values = HashSet::new();

        for index in range {
            if let Some(number) = self.cell_state[*index].number {
                if values.contains(&number) {
                    return false;
                }
                values.insert(number);
            }
        }

        true
    }

    fn highlight(&mut self) {
        self.clear_cell_selection();

        if self.selected_index.is_none() {
            return;
        }

        let sel_index = self.selected_index.unwrap();
        self.cell_state[sel_index].selection = CellSelection::Selected;

        let mut highlight_list = vec![sel_index];

        // only highlight numbers if the selected cell has a number
        if self.selected_number.is_some() {
            for (i, cell) in self.cell_state.iter_mut().enumerate() {
                if i != sel_index && cell.number == self.selected_number {
                    cell.selection = CellSelection::Emphasized;
                    highlight_list.push(i);
                }
            }
        }

        for index in highlight_list {
            self.highlight_areas(BOX_INDEXES, index);
            self.highlight_areas(ROW_INDEXES, index);
            self.highlight_areas(COLUMN_INDEXES, index);
        }
    }

    fn highlight_areas(&mut self, area: &[[usize; 9]; 9], selected_index: usize) {
        for index_row in area.iter() {
            if index_row.contains(&selected_index) {
                for index in index_row.iter() {
                    let selection = &mut self.cell_state[*index].selection;
                    if *selection == CellSelection::None {
                        *selection = CellSelection::Highlighted;
                    }
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

        for (i, cell) in self.cell_location.iter_mut().enumerate() {
            let (x, y) = index_to_xy(i, DIGIT_COUNT);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }
}
