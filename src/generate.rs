use macroquad::rand::gen_range;

fn shuffle<T>(array: &mut [T]) {
    let mut i = array.len();
    while i >= 2 {
        i -= 1;
        let j = gen_range(0, i);
        array.swap(i, j);
    }
}

pub fn generate_board(board: &mut [[u8; 9]; 9]) {
    fill_diagonal(board);
    solve(board);
}

fn fill_diagonal(board: &mut [[u8; 9]; 9]) {
    for i in (0..9).step_by(3) {
        fill_subgrid(board, i, i);
    }
}

fn fill_subgrid(board: &mut [[u8; 9]; 9], row: usize, col: usize) {
    let mut values: Vec<u8> = (1..=9).collect();
    shuffle(&mut values);

    for i in 0..3 {
        for j in 0..3 {
            board[row + i][col + j] = values.pop().unwrap();
        }
    }
}

fn solve(board: &mut [[u8; 9]; 9]) -> bool {
    if let Some((row, col)) = find_empty_cell(board) {
        let mut values: Vec<u8> = (1..=9).collect();
        shuffle(&mut values);

        for value in values {
            if is_valid(board, row, col, value) {
                board[row][col] = value;

                if solve(board) {
                    return true;
                }

                board[row][col] = 0; // Backtrack
            }
        }

        return false;
    }

    true
}

fn find_empty_cell(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    for (row, row_values) in board.iter().enumerate() {
        for (col, &value) in row_values.iter().enumerate() {
            if value == 0 {
                return Some((row, col));
            }
        }
    }
    None
}

fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, value: u8) -> bool {
    // Check row and column
    for i in 0..9 {
        if board[row][i] == value || board[i][col] == value {
            return false;
        }
    }

    // Check subgrid
    let subgrid_row = 3 * (row / 3);
    let subgrid_col = 3 * (col / 3);
    for i in 0..3 {
        for j in 0..3 {
            if board[subgrid_row + i][subgrid_col + j] == value {
                return false;
            }
        }
    }

    true
}

// 2nd part of the puzzle generation algorithm

pub fn create_puzzle(board: &mut [[u8; 9]; 9], difficulty: u8) {
    // Determine the number of cells to remove based on difficulty
    let num_cells_to_remove = match difficulty {
        1 => 35, // Easy
        2 => 45, // Medium
        3 => 50, // Hard
        _ => panic!("Invalid difficulty level!"),
    };

    // Remove cells randomly until the desired number is reached
    let mut cells_removed = 0;
    while cells_removed < num_cells_to_remove {
        let row = gen_range(0, 9);
        let col = gen_range(0, 9);

        if board[row][col] != 0 {
            // Backup the cell value and temporarily remove it
            let backup = board[row][col];
            board[row][col] = 0;

            // Check if the puzzle is still solvable with the current configuration
            let mut temp_board = *board;
            if has_unique_solution(&mut temp_board) {
                cells_removed += 1;
            } else {
                // If the puzzle is not unique, restore the backup value
                board[row][col] = backup;
            }
        }
    }
}

fn has_unique_solution(board: &mut [[u8; 9]; 9]) -> bool {
    let mut solutions = 0;
    solve_with_unique_solution(board, &mut solutions);
    solutions == 1
}

fn solve_with_unique_solution(board: &mut [[u8; 9]; 9], solution_count: &mut usize) {
    if let Some((row, col)) = find_empty_cell(board) {
        let mut values: Vec<u8> = (1..=9).collect();
        shuffle(&mut values);

        for value in values {
            if is_valid(board, row, col, value) {
                board[row][col] = value;

                if *solution_count < 2 {
                    solve_with_unique_solution(board, solution_count);
                    if *solution_count > 1 {
                        return;
                    }
                }

                board[row][col] = 0; // Backtrack
            }
        }

        return;
    }

    *solution_count += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_easy() {
        let mut board = [[0; 9]; 9];
        generate_board(&mut board);

        create_puzzle(&mut board, 1); // Easy difficulty

        // Count the number of filled cells
        let num_filled_cells = board.iter().flatten().filter(|&&value| value != 0).count();

        // Check that the number of filled cells matches the expected difficulty level
        assert_eq!(num_filled_cells, 81 - 35); // 81 is the total number of cells in the board

        // Check that the puzzle is still solvable and has a unique solution
        let mut puzzle_board = board;
        assert!(has_unique_solution(&mut puzzle_board));
    }

    #[test]
    fn test_create_medium() {
        let mut board = [[0; 9]; 9];
        generate_board(&mut board);

        create_puzzle(&mut board, 2); // Medium difficulty

        // Count the number of filled cells
        let num_filled_cells = board.iter().flatten().filter(|&&value| value != 0).count();

        // Check that the number of filled cells matches the expected difficulty level
        assert_eq!(num_filled_cells, 81 - 45); // 81 is the total number of cells in the board

        // Check that the puzzle is still solvable and has a unique solution
        let mut puzzle_board = board;
        assert!(has_unique_solution(&mut puzzle_board));
    }

    #[test]
    fn test_create_hard() {
        let mut board = [[0; 9]; 9];
        generate_board(&mut board);

        create_puzzle(&mut board, 3); // Hard difficulty

        // Count the number of filled cells
        let num_filled_cells = board.iter().flatten().filter(|&&value| value != 0).count();

        // Check that the number of filled cells matches the expected difficulty level
        assert_eq!(num_filled_cells, 81 - 50); // 81 is the total number of cells in the board

        // Check that the puzzle is still solvable and has a unique solution
        let mut puzzle_board = board;
        assert!(has_unique_solution(&mut puzzle_board));
    }
}
