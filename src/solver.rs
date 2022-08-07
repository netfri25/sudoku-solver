// mod solver


/// the trait with the abilities to solve a sudoku board
pub trait Solver {
    const WIDTH: usize = 9;
    const HEIGHT: usize = 9;

    /// checks if a value is in a given row
    fn in_row(&self, row: usize, value: u8) -> bool;

    /// checks if a value is in a given column
    fn in_col(&self, col: usize, value: u8) -> bool;

    /// checks if a value is inside a 3x3 box
    /// @param loc: range of 0..=2 on both loc.0 and loc.1
    fn in_box(&self, loc: (usize,usize), value: u8) -> bool;

    /// checks if the given location is a valid location for the value
    fn is_valid(&self, loc: (usize, usize), value: u8) -> bool;

    /// solves the sudoku. returns false on failure and true on success
    fn solve(&mut self) -> bool;
}
