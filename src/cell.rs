#[derive(Clone)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub number: Option<u32>,
    pub pencil: [Option<u32>; 9],
    pub selected: bool,
    pub emphasize: bool,
    pub highlighted: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            x: 0.0,
            y: 0.0,
            size: 0.0,
            number: None,
            pencil: [None, None, None, None, None, None, None, None, None],
            selected: false,
            emphasize: false,
            highlighted: false,
        }
    }

    pub fn new_init(x: f32, y: f32, size: f32, number: Option<u32>) -> Self {
        Cell {
            x,
            y,
            size,
            number,
            pencil: [None, None, None, None, None, None, None, None, None],
            selected: false,
            emphasize: false,
            highlighted: false,
        }
    }

    pub fn clear_highlight(&mut self) {
        self.selected = false;
        self.emphasize = false;
        self.highlighted = false;
    }

    pub fn click(&mut self, x: f32, y: f32) -> (bool, Option<u32>) {
        self.selected = false;

        if x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size {
            self.selected = true;
        }

        (self.selected, self.number)
    }

    pub fn has_pencil(&self) -> bool {
        self.pencil.iter().any(|&number| number.is_some())
    }

    pub fn set_pencil(&mut self, number: u32) {
        if !(1..=9).contains(&number) {
            return;
        }

        self.clear_number();
        self.pencil[number as usize - 1] = Some(number);
    }

    pub fn remove_pencil(&mut self, number: u32) {
        if !(1..=9).contains(&number) {
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

    pub fn is_number(&self, number: u32) -> bool {
        self.number == Some(number)
    }

    pub fn set_number(&mut self, number: u32) {
        if !(1..=9).contains(&number) {
            return;
        }

        self.clear_pencil();
        self.number = Some(number);
    }

    pub fn clear_number(&mut self) {
        self.number = None;
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;

    fn init_assert(cell: &Cell) {
        assert_eq!(cell.x, 0.0);
        assert_eq!(cell.y, 0.0);
        assert_eq!(cell.size, 0.0);
        assert_eq!(cell.number, None);
        assert!(!cell.selected);
    }

    fn click_assert(cell: &mut Cell, x: f32, y: f32, number: Option<u32>, expected: bool) {
        assert_eq!(cell.click(x, y), (expected, number));
        assert_eq!(cell.selected, expected);
    }

    #[test]
    fn cell_init() {
        let cell = Cell::new();
        init_assert(&cell);
    }

    #[test]
    fn cell_clear() {
        let mut cell = Cell::new();
        init_assert(&cell);

        cell.clear_highlight();
        assert_eq!(cell.selected, false);
    }

    #[test]
    fn cell_click() {
        let mut cell = Cell::new();
        init_assert(&cell);

        click_assert(&mut cell, 1.0, 1.0, None, false);
        cell.update(0.0, 0.0, 32.0);
        click_assert(&mut cell, 1.0, 1.0, None, true);
        click_assert(&mut cell, 32.0, 32.0, None, true);
        click_assert(&mut cell, 32.1, 32.1, None, false);

        cell.update(0.0, 0.0, 64.0);
        click_assert(&mut cell, 1.0, 1.0, None, true);
        click_assert(&mut cell, 32.0, 32.0, None, true);
        click_assert(&mut cell, 32.1, 32.1, None, true);
        click_assert(&mut cell, 64.0, 64.0, None, true);
        click_assert(&mut cell, 64.01, 64.01, None, false);

        cell.update(32.0, 32.0, 32.0);
        click_assert(&mut cell, 0.0, 0.0, None, false);
        click_assert(&mut cell, 1.0, 1.0, None, false);
        click_assert(&mut cell, 31.99, 31.99, None, false);
        click_assert(&mut cell, 32.0, 32.0, None, true);
        click_assert(&mut cell, 32.1, 32.1, None, true);
        click_assert(&mut cell, 64.0, 64.0, None, true);
        click_assert(&mut cell, 64.01, 64.01, None, false);
    }

    #[test]
    fn cell_click_value() {
        let mut cell_none = Cell::new_init(32.0, 32.0, 32.0, None);
        click_assert(&mut cell_none, 41.0, 41.0, None, true);

        let mut cell_number = Cell::new_init(32.0, 32.0, 32.0, Some(1));
        click_assert(&mut cell_number, 41.0, 41.0, Some(1), true);
        click_assert(&mut cell_number, 11.0, 11.0, Some(1), false);

        cell_number.set_number(9);
        click_assert(&mut cell_number, 41.0, 41.0, Some(9), true);
        click_assert(&mut cell_number, 11.0, 11.0, Some(9), false);

        cell_number.clear_number();
        click_assert(&mut cell_number, 41.0, 41.0, None, true);
        click_assert(&mut cell_number, 11.0, 11.0, None, false);
    }

    #[test]
    fn pencil_test() {
        let mut cell = Cell::new_init(0.0, 0.0, 32.0, None);
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
        cell.clear_pencil();
        assert!(!cell.has_pencil());

        for i in 0..9 {
            assert!(!cell.has_pencil());
            
            cell.set_pencil(i + 1);
            assert!(cell.has_pencil());
            assert_eq!(cell.pencil[i as usize], Some(i + 1));

            cell.remove_pencil(i + 1);
            assert_eq!(cell.pencil[i as usize], None);
        }
    }
}
