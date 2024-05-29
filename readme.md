This is a re-implementation of BYUI's cse 124 project 4

# Usage
- `cd sudoku` (Navigate to this repo)
- `cargo run`
- When prompted for the board's location type any path to a valid save file
  - `./saves/myGame.txt` and `./saves/complete.txt` are valid save files

# Save Format
- Rows are delimited by `\n`
- Values are delimited on a row by whitespace
- Values must be from 0..=9
- A Value of 0 represents an empty space