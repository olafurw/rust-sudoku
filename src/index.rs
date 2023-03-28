pub fn index_to_xy(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

pub fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    width * y + x
}

#[cfg(test)]
mod tests {
    use super::{index_to_xy, xy_to_index};

    #[test]
    fn index_to_xy_test() {
        assert_eq!(index_to_xy(1, 1), (0, 1));
        assert_eq!(index_to_xy(2, 1), (0, 2));
        assert_eq!(index_to_xy(2, 9), (2, 0));
        assert_eq!(index_to_xy(3, 9), (3, 0));
        assert_eq!(index_to_xy(9, 9), (0, 1));
        assert_eq!(index_to_xy(10, 9), (1, 1));
    }

    #[test]
    fn xy_to_index_test() {
        assert_eq!(xy_to_index(0, 0, 1), 0);
        assert_eq!(xy_to_index(0, 1, 1), 1);
        assert_eq!(xy_to_index(1, 0, 1), 1);
        assert_eq!(xy_to_index(1, 1, 1), 2);
        assert_eq!(xy_to_index(0, 0, 9), 0);
        assert_eq!(xy_to_index(1, 0, 9), 1);
        assert_eq!(xy_to_index(0, 1, 9), 9);
        assert_eq!(xy_to_index(2, 2, 9), 20);
        
    }
}