// mod board

use crate::solver::Solver;

type Cell = u8;

#[derive(Default)]
pub struct Board {
    content: [[Cell; 9]; 9],
}

impl Solver for Board {
    const WIDTH: usize = 9;
    const HEIGHT: usize = 9;

    /// tells if the value is already at the given row
    fn in_row(&self, row: usize, value: u8) -> bool {
        (0..row).any(|i| self.content[row][i] == value)
    }

    /// tells if the value is already in the given column
    fn in_col(&self, col: usize, value: u8) -> bool {
        (0..col).any(|i| self.content[i][col] == value)
    }

    /// tells if the value is already in a specified location of a 3x3 box,
    /// where the loc is the location of the box, not the value.
    fn in_box(&self, loc: (usize, usize), value: u8) -> bool {
        let x = loc.0 * 3;
        let y = loc.1 * 3;

        for i in y..y + 3 {
            for j in x..x + 3 {
                if self.content[i][j] == value {
                    return true;
                }
            }
        }

        false
    }

    /// tells if its ok to put value at loc inside the board
    fn is_valid(&self, loc: (usize, usize), value: u8) -> bool {
        !self.in_row(loc.1, value)
            && !self.in_col(loc.0, value)
            && !self.in_box((loc.0 / 3, loc.1 / 3), value)
    }

    /// solves the board
    fn solve(&mut self) -> bool {
        fn solve_helper(board: &mut Board, loc: (usize, usize)) -> bool {
            match loc {
                // after the last cell
                (9, 8) => return true,

                // when reached the end of the row
                (9, y) => return solve_helper(board, (0, y + 1)),

                // don't modify the board if its value is already set
                (x, y) if board.content[y][x] != 0 => return solve_helper(board, (x + 1, y)),

                // continue to the code after the match statment
                _ => {}
            }

            for n in 1..=9 {
                if !board.is_valid(loc, n) {
                    continue
                }

                board.content[loc.1][loc.0] = n;
                if solve_helper(board, (loc.0 + 1, loc.1)) {
                    return true;
                }
            }

            board.content[loc.1][loc.0] = 0;
            false
        }

        solve_helper(self, (0, 0))
    }
}

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    /// gives the Board that the BoardBuilder has built
    pub fn build(self) -> Board {
        self.board
    }

    /// creates an empty board builder
    pub fn new() -> Self {
        Self { board: Default::default() }
    }

    /// inserts value at loc inside the board
    pub fn insert(mut self, loc: (usize, usize), value: u8) -> Self {
        self.board.content[loc.1][loc.0] = value;
        self
    }
}


/// you shouldn't care about this stuff

impl From<[[Cell; <Board as Solver>::WIDTH]; <Board as Solver>::HEIGHT]> for BoardBuilder {
    fn from(arr: [[Cell; <Board as Solver>::WIDTH]; <Board as Solver>::HEIGHT]) -> BoardBuilder {
        BoardBuilder {
            board: Board {
                content: arr,
            },
        }
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..<Self as Solver>::HEIGHT {
            for i in 0..<Self as Solver>::WIDTH*2+1 {
                write!(f, "{}", ['+','-'][i%2])?;
            }
            writeln!(f)?;

            write!(f, "|")?;
            for x in 0..<Self as Solver>::WIDTH {
                let value = self.content[y][x];
                if value == 0 {
                    write!(f, " |")?;
                } else {
                    write!(f, "{}|", value)?;
                }
            }

            writeln!(f)?;
        }

        for i in 0..<Self as Solver>::WIDTH*2+1 {
            write!(f, "{}", ['+','-'][i%2])?;
        }
        writeln!(f)?;

        Ok(())
    }
}
