use std::collections::HashSet;

use crate::board_history::BoardHistory;
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
    pub history: BoardHistory,
    pub cell_state: [CellState; 81],
    pub cell_location: [CellLocation; 81],
    pub number_count: [u8; 9],
    pub mode: BoardMode,
    pub delete_mode: bool,
    pub board_size: f32,
    pub game_padding: f32,
    pub portrait: bool,
    pub cell_size: f32,
    pub selected_index: Option<usize>,
    pub selected_number: Option<u8>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            history: BoardHistory::new(),
            cell_state: [Default::default(); 81],
            cell_location: [Default::default(); 81],
            number_count: [0; 9],
            mode: BoardMode::Normal,
            delete_mode: false,
            board_size: 0.0,
            game_padding: 0.0,
            portrait: true,
            cell_size: 0.0,
            selected_index: None,
            selected_number: None,
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

    pub fn toggle_pencil_mode(&mut self) {
        match self.mode {
            BoardMode::Normal => self.mode = BoardMode::Pencil,
            BoardMode::Pencil => self.mode = BoardMode::Normal,
        }
    }

    pub fn toggle_delete_mode(&mut self) {
        self.delete_mode = !self.delete_mode;
    }

    pub fn disable_delete_mode(&mut self) {
        self.delete_mode = false;
    }

    pub fn undo(&mut self) {
        let undo_point = self.history.undo();
        if undo_point.is_none() {
            return;
        }

        let undo_point = undo_point.unwrap();

        self.cell_state = undo_point.cell_state;
        self.mode = undo_point.mode;
        self.selected_index = undo_point.selected_index;
        self.selected_number = undo_point.selected_number;

        self.highlight();
        self.update_number_count();
    }

    fn add_undo_point(&mut self) {
        self.history.add_undo_point(
            &self.cell_state,
            self.mode,
            self.selected_index,
            self.selected_number,
        );
    }

    fn clear_cell_selection(&mut self) {
        for cell in self.cell_state.iter_mut() {
            cell.clear_selection();
        }
    }

    pub fn set_selected_number(&mut self, number: u8) {
        if let Some(num) = self.selected_number {
            if num == number {
                return;
            }
        }

        self.selected_number = Some(number);
        self.add_undo_point();
    }

    pub fn is_victory(&self) -> bool {
        for count in self.number_count.iter() {
            if *count != DIGIT_COUNT {
                return false;
            }
        }
        true
    }

    pub fn is_number_done(&self, number: u8) -> bool {
        self.number_count[(number - 1) as usize] == DIGIT_COUNT
    }

    pub fn update_number_count(&mut self) {
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

        if self.delete_mode {
            if cell.has_number() && !cell.has_initial_number() {
                self.add_undo_point();
                self.selected_index = clicked_index;
                self.selected_number = self.cell_state[self.selected_index.unwrap()].number;
                self.cell_state[self.selected_index.unwrap()].clear_number();
                self.update_number_count();
                self.highlight();
            } else if cell.has_pencil() {
                self.add_undo_point();
                self.selected_index = clicked_index;
                self.cell_state[self.selected_index.unwrap()].clear_pencil();
                self.highlight();
            }

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
