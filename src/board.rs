use std::collections::HashSet;

use crate::cell_location::CellLocation;
use crate::cell_state::{CellSelection, CellState};
use crate::index::index_to_xy;
use crate::{
    is_legal_index, is_legal_number, BOX_INDEXES, COLUMN_INDEXES, DIGIT_COUNT, ROW_INDEXES,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardMode {
    Normal,
    Pencil,
}

pub struct Board {
    pub cell_state: [CellState; 81],
    pub cell_state_history: Vec<[CellState; 81]>,
    pub cell_location: [CellLocation; 81],
    pub number_count: [u8; 9],
    pub mode: BoardMode,
    pub mode_history: Vec<BoardMode>,
    pub board_size: f32,
    pub game_padding: f32,
    pub portrait: bool,
    pub cell_size: f32,
    pub selected_index: Option<usize>,
    pub selected_index_history: Vec<Option<usize>>,
    pub selected_number: Option<u8>,
    pub selected_number_history: Vec<Option<u8>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cell_state: [Default::default(); 81],
            cell_state_history: vec![],
            cell_location: [Default::default(); 81],
            number_count: [0; 9],
            mode: BoardMode::Normal,
            mode_history: vec![],
            board_size: 0.0,
            game_padding: 0.0,
            portrait: true,
            cell_size: 0.0,
            selected_index: None,
            selected_index_history: vec![],
            selected_number: None,
            selected_number_history: vec![],
        }
    }

    pub fn cell_initial_to_string(&self) -> String {
        let mut result = String::new();
        for index in 0..81 {
            let cell = self.cell_state[index];
            if cell.has_initial_number() {
                result.push_str(&cell.number.unwrap().to_string());
            } else {
                result.push('0');
            }
        }

        result
    }

    fn is_cell_selected(&self) -> bool {
        self.selected_index.is_some()
    }

    pub fn toggle_pencil_mode(&mut self) {
        match self.mode {
            BoardMode::Normal => self.mode = BoardMode::Pencil,
            BoardMode::Pencil => self.mode = BoardMode::Normal,
        }
    }

    pub fn undo(&mut self) {
        if self.cell_state_history.is_empty() {
            return;
        }

        self.cell_state = self.cell_state_history.pop().unwrap();
        self.mode = self.mode_history.pop().unwrap();
        self.selected_index_history.pop();
        self.selected_number_history.pop();

        if self.selected_index_history.is_empty() {
            self.selected_index = None;
            self.selected_number = None;
        } else {
            self.selected_index = *self.selected_index_history.last().unwrap();
            self.selected_number = *self.selected_number_history.last().unwrap();
        }

        self.highlight();
        self.update_number_count();
    }

    pub fn add_undo_point(&mut self) {
        self.cell_state_history.push(self.cell_state);
        self.mode_history.push(self.mode);
        self.selected_index_history.push(self.selected_index);
        self.selected_number_history.push(self.selected_number);
    }

    pub fn clear_cell_selection(&mut self) {
        for cell in self.cell_state.iter_mut() {
            cell.clear_selection();
        }
    }

    pub fn clear_number(&mut self) {
        if !self.is_cell_selected() {
            return;
        }

        self.cell_state[self.selected_index.unwrap()].clear_number();
        self.clear_cell_selection();
    }

    pub fn set_selected_number(&mut self, number: u8) {
        self.selected_number = Some(number);
    }

    pub fn is_number_done(&self, number: u8) -> bool {
        self.number_count[(number - 1) as usize] == DIGIT_COUNT
    }

    fn update_number_count(&mut self) {
        self.number_count = [0; 9];

        for cell in self.cell_state.iter() {
            if let Some(number) = cell.number {
                self.number_count[(number - 1) as usize] += 1;
            }
        }
    }

    fn can_insert(&self, index: Option<usize>, number: Option<u8>) -> bool {
        if index.is_none() || number.is_none() {
            return false;
        }

        let index = index.unwrap();
        if !is_legal_index(index) {
            return false;
        }

        let number = number.unwrap();
        if !is_legal_number(number) {
            return false;
        }

        let cell = &self.cell_state[index];
        !cell.is_number(number)
    }

    fn try_insert(&mut self, index: Option<usize>, number: Option<u8>) -> bool {
        if !self.can_insert(index, number) {
            return false;
        }

        let cell = &mut self.cell_state[index.unwrap()];
        cell.set_number(number.unwrap())
    }

    pub fn click(&mut self, x: f32, y: f32) {
        // Don't need to process clicks if we know they're outside the board
        if (self.portrait && y >= self.board_size + self.game_padding)
            || (!self.portrait && x >= self.board_size + self.game_padding)
        {
            return;
        }

        let mut clicked_index: Option<usize> = None;

        // perform a click on each cell to see which one
        // gets selected
        for i in 0..81 {
            let loc = &self.cell_location[i];
            let clicked = loc.click(x, y);
            if clicked {
                clicked_index = Some(i);
                break;
            }
        }

        // no cell was clicked
        if clicked_index.is_none() {
            self.selected_index = None;
            return;
        }

        // you can't change initial numbers
        let cell = &self.cell_state[clicked_index.unwrap()];
        if cell.has_initial_number() {
            return;
        }

        if self.mode == BoardMode::Normal {
            if !self.can_insert(clicked_index, self.selected_number) {
                return;
            }

            self.add_undo_point();
            self.selected_index = clicked_index;

            if !self.try_insert(self.selected_index, self.selected_number) {
                return;
            }

            if !self.is_valid() {
                self.undo();
                return;
            }

            self.highlight();
            self.clear_pencil(self.selected_number.unwrap());
            self.update_number_count();
        } else if self.mode == BoardMode::Pencil {
            if self.selected_number.is_none() || cell.has_number() {
                return;
            }

            let pencil_number = self.selected_number.unwrap();
            if cell.has_this_pencil(pencil_number) {
                self.add_undo_point();
                self.selected_index = clicked_index;

                self.cell_state[self.selected_index.unwrap()].remove_pencil(pencil_number);
            } else if cell.selection == CellSelection::None {
                self.add_undo_point();
                self.selected_index = clicked_index;

                self.cell_state[self.selected_index.unwrap()].set_pencil(pencil_number);
            }
        }
    }

    fn clear_pencil(&mut self, number: u8) {
        for cell in self.cell_state.iter_mut() {
            if cell.has_number() || cell.selection == CellSelection::Emphasized {
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

    pub fn highlight(&mut self) {
        self.clear_cell_selection();

        let mut highlight_list = vec![];

        // only highlight numbers if the selected cell has a number
        if self.selected_number.is_some() {
            for (i, cell) in self.cell_state.iter_mut().enumerate() {
                if cell.has_number() {
                    cell.selection = CellSelection::Highlighted;
                }
                if cell.number == self.selected_number {
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

    pub fn update(&mut self, board_size: f32, game_padding: f32, portrait: bool) -> bool {
        if self.board_size as i32 == board_size as i32 {
            return false;
        }

        self.board_size = board_size;
        self.game_padding = game_padding;
        self.portrait = portrait;
        self.cell_size = self.board_size / 9.0;

        for (i, cell) in self.cell_location.iter_mut().enumerate() {
            let (x, y) = index_to_xy(i, DIGIT_COUNT as usize);
            let x_pos = self.game_padding + (x as f32 * self.cell_size);
            let y_pos = self.game_padding + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_board() {}
}
