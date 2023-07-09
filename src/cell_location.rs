use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CellLocation {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl Default for CellLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl CellLocation {
    pub fn new() -> Self {
        CellLocation {
            x: 0.0,
            y: 0.0,
            size: 0.0,
        }
    }

    pub fn click(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_location::CellLocation;

    fn init_assert(cell: &CellLocation) {
        assert_eq!(cell.x, 0.0);
        assert_eq!(cell.y, 0.0);
        assert_eq!(cell.size, 0.0);
    }

    #[test]
    fn cell_default() {
        let cell = CellLocation::default();
        init_assert(&cell);
    }

    #[test]
    fn cell_init() {
        let cell = CellLocation::new();
        init_assert(&cell);
    }

    #[test]
    fn cell_click() {
        let mut cell = CellLocation::new();
        init_assert(&cell);

        assert!(!cell.click(1.0, 1.0));
        cell.update(0.0, 0.0, 32.0);
        assert!(cell.click(1.0, 1.0));
        assert!(cell.click(32.0, 32.0));
        assert!(!cell.click(32.1, 32.1));

        cell.update(0.0, 0.0, 64.0);
        assert!(cell.click(1.0, 1.0));
        assert!(cell.click(32.0, 32.0));
        assert!(cell.click(32.1, 32.1));
        assert!(cell.click(64.0, 64.0));
        assert!(!cell.click(64.01, 64.01));

        cell.update(32.0, 32.0, 32.0);
        assert!(!cell.click(0.0, 0.0));
        assert!(!cell.click(1.0, 1.0));
        assert!(!cell.click(31.99, 31.99));
        assert!(cell.click(32.0, 32.0));
        assert!(cell.click(32.1, 32.1));
        assert!(cell.click(64.0, 64.0));
        assert!(!cell.click(64.01, 64.01));
    }
}
