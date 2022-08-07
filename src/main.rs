// TODO: add graphics && GUI
mod solver;
mod board;
use solver::Solver;

fn main() {
    let mut board = board::BoardBuilder::new()
        .insert((3,0), 7)
        .insert((4,0), 9)
        .insert((7,0), 5)
        .build();

    println!("{:?}", board);

    if board.solve() {
        println!("board has been solved!\n{:?}", board);
    } else {
        println!("unable to solve the board.\n{:?}", board);
    }
}
