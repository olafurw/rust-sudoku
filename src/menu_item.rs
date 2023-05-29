#[derive(Clone, Copy)]
pub struct MenuItem {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuItem {
    pub fn new() -> Self {
        MenuItem {
            x: 0.0,
            y: 0.0,
            size: 0.0,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.size = size;
    }

    pub fn click(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size
    }
}

#[cfg(test)]
mod tests {
    use super::MenuItem;

    #[test]
    fn empty_menu_item() {
        let item = MenuItem::new();
        assert_eq!(item.x, 0.0);
        assert_eq!(item.y, 0.0);
        assert_eq!(item.size, 0.0);
    }

    #[test]
    fn default_menu_item() {
        let item = MenuItem::default();
        assert_eq!(item.x, 0.0);
        assert_eq!(item.y, 0.0);
        assert_eq!(item.size, 0.0);
    }

    #[test]
    fn menu_item_update() {
        let mut item = MenuItem::new();
        assert_eq!(item.x, 0.0);
        assert_eq!(item.y, 0.0);
        assert_eq!(item.size, 0.0);

        item.update(10.0, 11.0, 12.0);
        assert_eq!(item.x, 10.0);
        assert_eq!(item.y, 11.0);
        assert_eq!(item.size, 12.0);
    }

    #[test]
    fn menu_item_click() {
        let mut item = MenuItem::new();
        assert_eq!(item.x, 0.0);
        assert_eq!(item.y, 0.0);
        assert_eq!(item.size, 0.0);

        item.update(10.0, 11.0, 12.0);
        assert_eq!(item.x, 10.0);
        assert_eq!(item.y, 11.0);
        assert_eq!(item.size, 12.0);

        assert!(!item.click(0.0, 0.0));
        assert!(item.click(10.0, 11.0));
        assert!(item.click(11.0, 12.0));
        assert!(item.click(22.0, 23.0));
        assert!(!item.click(23.0, 24.0));
    }
}
