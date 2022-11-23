use rust_sudoku::run;

fn main() {
    pollster::block_on(run());
}