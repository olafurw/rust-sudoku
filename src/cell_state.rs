use crate::is_legal_number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellSelection {
    None,
    Emphasized,
    Highlighted,
}

#[derive(Clone, Copy)]
pub struct CellState {
    pub number: Option<u8>,
    pub pencil: [Option<u8>; 9],
    pub selection: CellSelection,
    pub initial: bool,
}

impl Default for CellState {
    fn default() -> Self {
        Self::new()
    }
}

impl CellState {
    pub fn new() -> Self {
        CellState {
            number: None,
            pencil: [None, None, None, None, None, None, None, None, None],
            selection: CellSelection::None,
            initial: false,
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = CellSelection::None;
    }

    pub fn has_pencil(&self) -> bool {
        self.pencil.iter().any(|&number| number.is_some())
    }

    pub fn has_this_pencil(&self, number: u8) -> bool {
        self.pencil[number as usize - 1].is_some()
    }

    pub fn set_pencil(&mut self, number: u8) {
        if self.initial || !is_legal_number(number) {
            return;
        }

        self.clear_number();
        self.pencil[number as usize - 1] = Some(number);
    }

    pub fn remove_pencil(&mut self, number: u8) {
        if self.initial || !is_legal_number(number) {
            return;
        }

        self.clear_number();
        self.pencil[number as usize - 1] = None;
    }

    pub fn clear_pencil(&mut self) {
        self.pencil = [None, None, None, None, None, None, None, None, None];
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    pub fn is_number(&self, number: u8) -> bool {
        self.number == Some(number)
    }

    pub fn set_initial_number(&mut self, number: u8) {
        if self.initial || !is_legal_number(number) {
            return;
        }

        self.set_number(number);
        self.initial = true;
    }

    pub fn set_number(&mut self, number: u8) {
        if self.initial || !is_legal_number(number) {
            return;
        }

        self.clear_pencil();
        self.number = Some(number);
    }

    pub fn clear_number(&mut self) {
        if self.initial {
            return;
        }

        self.number = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_state::{CellSelection, CellState};

    fn init_assert(cell: &CellState) {
        assert_eq!(cell.number, None);
        assert!(cell.selection == CellSelection::None);
    }

    #[test]
    fn cell_default() {
        let cell = CellState::default();
        init_assert(&cell);
    }

    #[test]
    fn cell_init() {
        let cell = CellState::new();
        init_assert(&cell);
    }

    #[test]
    fn cell_clear() {
        let mut cell = CellState::new();
        init_assert(&cell);

        cell.clear_selection();
        assert!(cell.selection == CellSelection::None);
    }

    #[test]
    fn number_test() {
        let mut cell = CellState::new();
        assert!(!cell.has_number());
        assert!(!cell.is_number(1));

        cell.set_number(1);
        assert!(cell.has_number());
        assert!(cell.is_number(1));
        assert!(!cell.is_number(2));

        cell.set_number(2);
        assert!(cell.has_number());
        assert!(cell.is_number(2));
        assert!(!cell.is_number(1));

        cell.set_number(12);
        assert!(cell.has_number());
        assert!(cell.is_number(2));
        assert!(!cell.is_number(1));

        cell.clear_number();
        assert!(!cell.has_number());
        assert!(!cell.is_number(1));

        cell.set_initial_number(1);
        assert!(cell.has_number());
        assert!(cell.is_number(1));
        assert!(!cell.is_number(2));

        cell.set_initial_number(2);
        assert!(cell.has_number());
        assert!(cell.is_number(1));
        assert!(!cell.is_number(2));

        cell.clear_number();
        assert!(cell.has_number());
        assert!(cell.is_number(1));
        assert!(!cell.is_number(2));
    }

    #[test]
    fn pencil_test() {
        let mut cell = CellState::new();
        assert!(!cell.has_pencil());

        cell.clear_pencil();
        assert!(!cell.has_pencil());

        cell.remove_pencil(1);
        assert!(!cell.has_pencil());

        cell.set_pencil(1);
        assert!(cell.has_pencil());
        assert_eq!(cell.pencil[0], Some(1));

        cell.remove_pencil(1);
        assert!(!cell.has_pencil());

        cell.set_pencil(1);
        assert!(cell.has_pencil());
        assert!(cell.has_this_pencil(1));
        assert!(!cell.has_this_pencil(2));

        cell.clear_pencil();
        assert!(!cell.has_pencil());
        assert!(!cell.has_this_pencil(1));
        assert!(!cell.has_this_pencil(2));

        cell.set_number(1);
        assert!(cell.has_number());
        assert!(!cell.has_pencil());

        cell.set_pencil(1);
        assert!(!cell.has_number());
        assert!(cell.has_pencil());

        cell.clear_pencil();

        for i in 0..9 {
            assert!(!cell.has_pencil());

            cell.set_pencil(i + 1);
            assert!(cell.has_pencil());
            assert_eq!(cell.pencil[i as usize], Some(i + 1));

            cell.remove_pencil(i + 1);
            assert_eq!(cell.pencil[i as usize], None);
        }

        cell.clear_pencil();
        assert!(!cell.has_pencil());
        assert!(!cell.has_number());

        cell.set_initial_number(1);
        assert!(cell.has_number());
        assert!(!cell.has_pencil());
        assert!(cell.is_number(1));

        cell.set_pencil(2);
        assert!(cell.has_number());
        assert!(!cell.has_pencil());
        assert!(cell.is_number(1));

        cell.remove_pencil(2);
        assert!(cell.has_number());
        assert!(!cell.has_pencil());
        assert!(cell.is_number(1));
    }
}
