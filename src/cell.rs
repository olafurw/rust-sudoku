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
        self.selected = false;

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

#[cfg(test)]
mod tests {
    use crate::cell::Cell;

    fn init_assert(cell: &Cell) {
        assert_eq!(cell.x, 0.0);
        assert_eq!(cell.y, 0.0);
        assert_eq!(cell.size, 0.0);
        assert_eq!(cell.number, Some(1));
        assert_eq!(cell.selected, false);
    }

    fn click_assert(cell: &mut Cell, x: f32, y: f32, expected: bool) {
        assert_eq!(cell.click(x, y), expected);
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

        cell.clear();
        assert_eq!(cell.selected, false);
    }

    #[test]
    fn cell_click() {
        let mut cell = Cell::new();
        init_assert(&cell);

        click_assert(&mut cell, 1.0, 1.0, false);
        cell.update(0.0, 0.0, 32.0);
        click_assert(&mut cell, 1.0, 1.0, true);
        click_assert(&mut cell, 32.0, 32.0, true);
        click_assert(&mut cell, 32.1, 32.1, false);

        cell.update(0.0, 0.0, 64.0);
        click_assert(&mut cell, 1.0, 1.0, true);
        click_assert(&mut cell, 32.0, 32.0, true);
        click_assert(&mut cell, 32.1, 32.1, true);
        click_assert(&mut cell, 64.0, 64.0, true);
        click_assert(&mut cell, 64.01, 64.01, false);

        cell.update(32.0, 32.0, 32.0);
        click_assert(&mut cell, 0.0, 0.0, false);
        click_assert(&mut cell, 1.0, 1.0, false);
        click_assert(&mut cell, 31.99, 31.99, false);
        click_assert(&mut cell, 32.0, 32.0, true);
        click_assert(&mut cell, 32.1, 32.1, true);
        click_assert(&mut cell, 64.0, 64.0, true);
        click_assert(&mut cell, 64.01, 64.01, false);
    }
}