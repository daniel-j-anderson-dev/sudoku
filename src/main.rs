mod game;
use game::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_input("Where is your board located? ")?;
    let mut board = Board::read_from_file(path)?;

    println!("{}\n{}", menu::OPTIONS, board);

    loop {
        let menu_option = get_parsed_input::<MenuOption>("> ")?;

        match menu_option {
            MenuOption::ShowInstructions => print!("{}", menu::OPTIONS),
            MenuOption::DisplayBoard => print!("{}", board),
            MenuOption::EditOneSquare => {
                let index = get_parsed_input("What are the coordinates of the square: ")?;

                if !board.get(index).is_empty() {
                    println!("\nERROR: Square '{}' is filled\n", index);
                    continue;
                }

                let value = get_parsed_input(&format!("What is the value at '{}': ", index))?;

                if !board
                    .possible_values(index)
                    .any(|possible_value| possible_value == value)
                {
                    println!(
                        "\nERROR: Value '{}' in square '{}' is invalid\n",
                        value, index
                    );
                    continue;
                }

                *board.get_mut(index) = value;
            }
            MenuOption::ShowPossibleValues => {
                let index = get_parsed_input("What are the coordinates of the square: ")?;
                let possible_values = board.possible_values(index).collect::<Vec<_>>();

                print!("The possible values for '{}' are: ", index);
                for (i, possible_value) in possible_values.iter().enumerate() {
                    print!("{}", possible_value);
                    if i < possible_values.len() - 1 {
                        print!(", ");
                    }
                }

                println!();
            }
            MenuOption::SaveAndQuit => {
                let path = get_input("What file would you like to write your board to: ")?;
                board.save_to_file(path)?;
                println!("Board written successfully");
                break;
            }
        }

        println!();
    }

    return Ok(());
}
