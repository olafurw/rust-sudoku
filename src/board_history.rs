use crate::{board::BoardMode, cell_state::CellState};

pub struct BoardUndoPoint {
    pub cell_state: [CellState; 81],
    pub mode: BoardMode,
    pub selected_index: Option<usize>,
    pub selected_number: Option<u8>,
}

pub struct BoardHistory {
    pub cell_state_history: Vec<[CellState; 81]>,
    pub mode_history: Vec<BoardMode>,
    pub selected_index_history: Vec<Option<usize>>,
    pub selected_number_history: Vec<Option<u8>>,
}

impl BoardHistory {
    pub fn new() -> Self {
        BoardHistory {
            cell_state_history: vec![],
            mode_history: vec![],
            selected_index_history: vec![],
            selected_number_history: vec![],
        }
    }

    pub fn undo(&mut self) -> Option<BoardUndoPoint> {
        if self.cell_state_history.is_empty() {
            return None;
        }

        let cell_state = self.cell_state_history.pop().unwrap();
        let mode = self.mode_history.pop().unwrap();

        self.selected_index_history.pop();
        let selected_index = if self.selected_index_history.is_empty() {
            None
        } else {
            *self.selected_index_history.last().unwrap()
        };

        self.selected_number_history.pop();
        let selected_number = if self.selected_number_history.is_empty() {
            None
        } else {
            *self.selected_number_history.last().unwrap()
        };

        Some(BoardUndoPoint {
            cell_state,
            mode,
            selected_index,
            selected_number,
        })
    }

    pub fn add_undo_point(
        &mut self,
        cell_states: &[CellState; 81],
        mode: BoardMode,
        selected_index: Option<usize>,
        selected_number: Option<u8>,
    ) {
        self.cell_state_history.push(*cell_states);
        self.mode_history.push(mode);
        self.selected_index_history.push(selected_index);
        self.selected_number_history.push(selected_number);
    }
}
