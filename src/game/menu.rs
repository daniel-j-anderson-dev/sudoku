pub const OPTIONS: &str = "Options:\n? Show these instructions\nD Display the board\nE Edit one square\nS Show the possible values for a square\nQ Save and quit\n";

pub enum MenuOption {
    ShowInstructions,
    DisplayBoard,
    EditOneSquare,
    ShowPossibleValues,
    SaveAndQuit,
}
impl std::str::FromStr for MenuOption {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "?" => Ok(Self::ShowInstructions),
            "D" => Ok(Self::DisplayBoard),
            "E" => Ok(Self::EditOneSquare),
            "S" => Ok(Self::ShowPossibleValues),
            "Q" => Ok(Self::SaveAndQuit),
            _ => Err("invalid command"),
        }
    }
}