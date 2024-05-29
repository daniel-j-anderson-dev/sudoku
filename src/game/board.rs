use crate::game::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Board {
    values: [[Value; 9]; 9],
}
impl Board {
    pub fn get(&self, index: Index) -> &Value {
        return &self.values[index.row][index.column];
    }

    pub fn get_mut(&mut self, index: Index) -> &mut Value {
        return &mut self.values[index.row][index.column];
    }

    pub fn row(&self, index: Index) -> impl Iterator<Item = &Value> {
        return self.values[index.row].iter();
    }

    pub fn column(&self, index: Index) -> impl Iterator<Item = &Value> {
        return (0..9).map(move |row_index| &self.values[row_index][index.column]);
    }

    pub fn sub_box(&self, index: Index) -> impl Iterator<Item = &Value> {
        return SubBox::from(index)
            .all_indexes()
            .map(|index| self.get(index));
    }

    pub fn possible_values(&self, index: Index) -> impl Iterator<Item = Value> {
        let mut values_seen = [true; 10];

        // check row
        for value in self.row(index).map(usize::from) {
            values_seen[value] = false;
        }

        // check column
        for value in self.column(index).map(usize::from) {
            values_seen[value] = false;
        }

        // check 3 by 3 box
        for value in self.sub_box(index).map(usize::from) {
            values_seen[value] = false;
        }

        return values_seen
            .into_iter()
            .enumerate()
            .filter_map(|(value, value_seen)| {
                if value_seen && value != 0 {
                    Some(value.into())
                } else {
                    None
                }
            });
    }

    pub fn save_data(&self) -> String {
        let mut output = String::new();
        for row in self.values.iter() {
            for value in row {
                output.push_str(&format!("{} ", value));
            }
            output.push('\n');
        }
        return output;
    }
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  A B C D E F G H I\n")?;

        for (row_index, row) in self.values.iter().enumerate() {
            write!(f, "{} ", row_index + 1)?;

            for (column_index, value) in row.iter().enumerate() {
                write!(f, "{}", value)?;

                if column_index % 3 == 2 && column_index != 8 {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
            }

            if row_index % 3 == 2 && row_index != 8 {
                write!(f, "\n  -----+-----+-----")?;
            }

            write!(f, "\n")?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBoardError {
    line_number: usize,
    message: String,
}
impl std::fmt::Display for ParseBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Could not parse board\nReason: {}\nline: {}",
            self.message, self.line_number
        );
    }
}
impl std::error::Error for ParseBoardError {}
impl std::str::FromStr for Board {
    type Err = ParseBoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::default();

        for (row_index, line) in s.lines().enumerate() {
            if row_index >= 9 {
                Err(ParseBoardError {
                    line_number: row_index + 1,
                    message: String::from("too many lines; must have 9 lines"),
                })?;
            }

            for (column_index, value) in line.split_whitespace().enumerate() {
                if column_index >= 9 {
                    Err(ParseBoardError {
                        line_number: row_index + 1,
                        message: String::from("too many values on line"),
                    })?;
                }
                let value = value.parse::<Value>().map_err(|message| ParseBoardError {
                    line_number: row_index + 1,
                    message,
                })?;
                board.values[row_index][column_index] = value;
            }
        }

        return Ok(board);
    }
}

/// ## Invariant
/// An instance of [Index]'s row_index and column_index will be between '0..=8'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Index {
    row: usize,
    column: usize,
}
impl Index {
    pub const fn new(row: usize, column: usize) -> Option<Self> {
        return if row < 9 && column < 9 {
            Some(Self { row, column })
        } else {
            None
        };
    }
}
impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = match self.row {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            8 => '9',
            _ => unreachable!("Index invariant"),
        };

        let column = match self.column {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            _ => unreachable!("Index invariant"),
        };

        return write!(f, "{}{}", column, row);
    }
}
impl std::str::FromStr for Index {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() > 2 {
            Err("too many characters to identify a Square")?
        }

        let s = s.to_uppercase();

        let column = match s.chars().nth(0) {
            Some('A') => 0,
            Some('B') => 1,
            Some('C') => 2,
            Some('D') => 3,
            Some('E') => 4,
            Some('F') => 5,
            Some('G') => 6,
            Some('H') => 7,
            Some('I') => 8,
            Some(c) => Err(format!("{} does not identify a valid column", c))?,
            None => Err("empty input")?,
        };

        let row = match s.chars().nth(1) {
            Some('1') => 0,
            Some('2') => 1,
            Some('3') => 2,
            Some('4') => 3,
            Some('5') => 4,
            Some('6') => 5,
            Some('7') => 6,
            Some('8') => 7,
            Some('9') => 8,
            Some(c) => Err(format!("{} does not identify a valid row", c))?,
            None => Err("too few characters to identify a Square")?,
        };

        let index = Index::new(row, column).expect("out of bounds error's handled");

        return Ok(index);
    }
}

fn cartesian_product<T: Copy, const N: usize>(
    outer: [T; N],
    inner: [T; N],
) -> impl Iterator<Item = (T, T)> {
    outer.into_iter().flat_map(move |outer_element| {
        inner
            .into_iter()
            .map(move |inner_element| (outer_element, inner_element))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubBox {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}
impl SubBox {
    pub fn all_indexes(&self) -> impl Iterator<Item = Index> {
        match self {
            SubBox::TopLeft => cartesian_product([0, 1, 2], [0, 1, 2]),
            SubBox::TopMiddle => cartesian_product([0, 1, 2], [3, 4, 5]),
            SubBox::TopRight => cartesian_product([0, 1, 2], [6, 7, 8]),
            SubBox::MiddleLeft => cartesian_product([3, 4, 5], [0, 1, 2]),
            SubBox::Center => cartesian_product([3, 4, 5], [3, 4, 5]),
            SubBox::MiddleRight => cartesian_product([3, 4, 5], [6, 7, 8]),
            SubBox::BottomLeft => cartesian_product([6, 7, 8], [0, 1, 2]),
            SubBox::BottomMiddle => cartesian_product([6, 7, 8], [3, 4, 5]),
            SubBox::BottomRight => cartesian_product([6, 7, 8], [6, 7, 8]),
        }
        .map(|(row, column)| {
            Index::new(row, column).expect("all args of cartesian_product calls are in bounds")
        })
    }
}
impl From<Index> for SubBox {
    fn from(value: Index) -> Self {
        if value.row <= 2 {
            if value.column <= 2 {
                Self::TopLeft
            } else if value.column <= 5 {
                Self::TopMiddle
            } else {
                Self::TopRight
            }
        } else if value.row <= 5 {
            if value.column <= 2 {
                Self::MiddleLeft
            } else if value.column <= 5 {
                Self::Center
            } else {
                Self::MiddleRight
            }
        } else {
            if value.column <= 2 {
                Self::BottomLeft
            } else if value.column <= 5 {
                Self::BottomMiddle
            } else {
                Self::BottomRight
            }
        }
    }
}
