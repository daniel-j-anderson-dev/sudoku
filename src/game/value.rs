#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Value {
    #[default]
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Value {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
}
impl From<usize> for Value {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => Self::Empty,
        }
    }
}
impl From<&Value> for usize {
    fn from(value: &Value) -> Self {
        match value {
            Value::Empty => 0,
            Value::One => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
        }
    }
}
impl From<&Value> for char {
    fn from(value: &Value) -> Self {
        match value {
            Value::Empty => ' ',
            Value::One => '1',
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
        }
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(self))?;
        return Ok(());
    }
}
impl std::str::FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.parse::<usize>().map_err(|e| e.to_string()) {
            Ok(n) => match n {
                0 => Ok(Self::Empty),
                1 => Ok(Self::One),
                2 => Ok(Self::Two),
                3 => Ok(Self::Three),
                4 => Ok(Self::Four),
                5 => Ok(Self::Five),
                6 => Ok(Self::Six),
                7 => Ok(Self::Seven),
                8 => Ok(Self::Eight),
                9 => Ok(Self::Nine),
                _ => Err(format!("value too large: {}", n)),
            },
            Err(e) => Err(e.into()),
        };
    }
}
