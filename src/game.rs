pub mod board;
pub mod value;
pub mod menu;

pub use crate::game::{
    board::{Board, Index},
    value::Value,
    menu::MenuOption,
};

/// Prompts the user for input then returns a line of input from the user
pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, BufRead, Write};

    let mut stdout = stdout().lock();
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;

    let mut input = String::new();
    stdin().lock().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    return Ok(input);
}

/// Prompts the user for input of a specific data-type then returns the value input by the user
pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        match get_input(prompt)?.parse() {
            Ok(parsed_input) => return Ok(parsed_input),
            Err(parse_error) => eprintln!("\nERROR: {}\n", parse_error),
        }
    }
}
