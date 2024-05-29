pub mod board;
pub mod value;
pub mod menu;

pub use crate::game::{
    board::{Board, Index},
    value::Value,
    menu::MenuOption,
};

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

pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        match get_input(prompt)?.parse() {
            Ok(parsed_input) => return Ok(parsed_input),
            Err(parse_error) => eprintln!("\nERROR: {}", parse_error),
        }
    }
}
