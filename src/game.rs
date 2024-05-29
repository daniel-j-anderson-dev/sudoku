pub mod board;
pub mod value;
pub mod menu;

pub use crate::game::{
    board::{Board, Index},
    value::Value,
    menu::MenuOption,
};

use std::{
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};

pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
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
            Err(parse_error) => eprintln!("\nERROR: {}\n", parse_error),
        }
    }
}

pub fn read_text_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::options().read(true).open(path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;
    let file_text = String::from_utf8(file_data)?;
    return Ok(file_text);
}

pub fn save_text_file(path: &str, data: &str) -> Result<(), std::io::Error> {
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(data.as_bytes())?;
    file.flush()?;
    return Ok(());
}
