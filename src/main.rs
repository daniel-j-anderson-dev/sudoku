mod game;
use game::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board_path = get_input("Where is your board located? ")?;
    let mut board = read_text_file(&board_path)?.parse::<Board>()?;

    println!("{}\n{}", menu::OPTIONS, board);

    loop {
        let input = get_parsed_input::<MenuOption>("> ")?;

        println!();

        match input {
            MenuOption::ShowInstructions => print!("{}", menu::OPTIONS),
            MenuOption::DisplayBoard => print!("{}", board),
            MenuOption::EditOneSquare => {
                let index = get_parsed_input("What are the coordinates of the square: ")?;

                if !board.get(index).is_empty() {
                    println!("\nERROR: Square '{}' is filled\n", index);
                    continue;
                }

                let value = get_parsed_input(&format!("\nWhat is the value at '{}': ", index))?;

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

                print!("\nThe possible values for '{}' are: ", index);
                for (i, possible_value) in possible_values.iter().enumerate() {
                    print!("{}", possible_value);
                    if i < possible_values.len() - 1 {
                        print!(", ");
                    }
                }

                println!();
            }
            MenuOption::SaveAndQuit => {
                let save_path = get_input("save prompt")?;
                let save_data = board.save_data();
                save_text_file(&save_path, &save_data)?;
                println!("\nBoard written successfully\n");
                break;
            }
        }

        println!();
    }

    return Ok(());
}
