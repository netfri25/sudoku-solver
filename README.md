Simple sudoku solver written in Rust. 

### Changing the board
In the main file, you can build a board using the BoardBuilder.
You can create the board like this:
```rust
fn main() {
   let mut board = BoardBuilder::new()
      .insert((0,0), 3)
      .insert((0,4), 6)
      .insert((3,5), 2)
      .build(); // you must call the build function at the end of the building process.

   if board.solve() {
      println!("the board has been solved successfully!");
      println!("{:?}", board)
   } else {
      println!("failed to solve the board :sad:")
   }
}
```

or like this:
```rust
fn main() {
   let mut board = BoardBuilder::from([
      [3,0,0,0,6,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,2,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
      [0,0,0,0,0,0,0,0,0],
   ]).build() // same here, you must call the build function.

   if board.solve() {
      println!("the board has been solved successfully!");
      println!("{:?}", board)
   } else {
      println!("failed to solve the board :sad:")
   }
}
```

both of them do the same exact thing, but in my opinion the first option is more handy.

### Running the code
You must have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.
then in your shell, navigate to this project folder and run:
```shell
cargo run --release
```
this should compile and execute the program.

### TODO:
   - [ ] add GUI and graphics.
   - [ ] add option to load boards from a file.
   - [ ] don't forget to complete the TODO list.
