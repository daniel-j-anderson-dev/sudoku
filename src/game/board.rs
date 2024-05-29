use crate::game::value::Value;

/// This data type represents a Sudoku [Board]. It is stored as a row major 9 by 9 array of [Value]s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Board {
    values: [[Value; 9]; 9],
}
impl Board {
    /// Create a new board based on the data at `path`
    pub fn read_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, Box<dyn std::error::Error>> {
        return Ok(std::fs::read_to_string(path)?.parse()?);
    }

    /// Save the current state of the board at 'path'
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let save_data = {
            let mut temp = String::new();
            for row in self.values.iter() {
                for value in row {
                    temp.push_str(&format!("{} ", value));
                }
                temp.push('\n');
            }
            temp
        };

        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        use std::io::Write;
        writeln!(file, "{}", save_data)?;

        return Ok(());
    }

    /// Returns a reference to the [Value] at `index`
    pub fn get(&self, index: Index) -> &Value {
        return &self.values[index.row][index.column];
    }
    
    /// Returns a mutable reference to the [Value] at `index`
    pub fn get_mut(&mut self, index: Index) -> &mut Value {
        return &mut self.values[index.row][index.column];
    }

    /// Returns an [Iterator] that yields all the elements in the same row as `index`
    pub fn row(&self, index: Index) -> impl Iterator<Item = &Value> {
        return self.values[index.row].iter();
    }
    
    /// Returns an [Iterator] that yields all the elements in the same column as `index`
    pub fn column(&self, index: Index) -> impl Iterator<Item = &Value> {
        return (0..9).map(move |row_index| &self.values[row_index][index.column]);
    }
    
    /// Returns an [Iterator] that yields all the elements in the same 3x3 box as `index`
    pub fn sub_box(&self, index: Index) -> impl Iterator<Item = &Value> {
        return SubBox::from(index)
        .all_indexes()
        .map(|index| self.get(index));
    }


    /// Returns an [Iterator] that yields all the possible [Value]s at `index`. If the [Value] at `index` is empty then there are no possible [Value]s
    pub fn possible_values(&self, index: Index) -> impl Iterator<Item = Value> {
        let is_empty = self.get(index).is_empty();

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
            .filter_map(move |(value, value_seen)| {
                if is_empty && value_seen && value != 0 {
                    Some(value.into())
                } else {
                    None
                }
            });
    }
}
impl std::fmt::Display for Board {
    /// Display the board as instructed
    /// ```text
    ///   A B C D E F G H I
    /// 1 7 2 3|     |1 5 9
    /// 2 6    |3   2|    8
    /// 3 8    |  1  |    2
    ///   -----+-----+-----
    /// 4   7  |6 5 4|  2
    /// 5     4|2   7|3
    /// 6   5  |9 3 1|  4
    ///   -----+-----+-----
    /// 7 5    |  7  |    3
    /// 8 4    |1   3|    6
    /// 9 9 3 2|     |7 1 4
    /// ```
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
impl std::str::FromStr for Board {
    type Err = String;
    /// Defines how to [str::parse] a string into a [Board]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::default();

        for (row_index, line) in s.lines().enumerate() {
            if row_index >= 9 {
                Err(format!("in Board string on line {} there are too many lines; must have 9 lines", row_index + 1))?;
            }

            for (column_index, value) in line.split_whitespace().enumerate() {
                if column_index >= 9 {
                    Err(format!("in Board string on line {} there are too many values", row_index + 1))?;
                }
                let value = value.parse::<Value>().map_err(|e| format!("in Board string on line {} value could not be parsed: {}", row_index + 1, e))?;
                board.values[row_index][column_index] = value;
            }
        }

        return Ok(board);
    }
}

/// This type represents an inbound index into the 9 by 9 sudoku board
///  ## Invariant
/// An instance of [Index]'s row_index and column_index will be between '0..=8'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Index {
    row: usize,
    column: usize,
}
impl Index {
    /// Create a new instance of [Index] if the indexes are inbounds 
    pub const fn new(row: usize, column: usize) -> Option<Self> {
        return if row < 9 && column < 9 {
            Some(Self { row, column })
        } else {
            None
        };
    }
}
impl std::fmt::Display for Index {
    /// Display the [Index] as described.
    /// example: `Index { row: 2, column: 4 }` == `"D3"`
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
    /// Defines how to [str::parse] an [Index] from a string
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

/// Returns an iterator that will yield each combination of each element of 'outer' and 'inner'. Has the same order as a nested loop hence the argument names.
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

/// This type represents a possible 3x3 [SubBox] in a Sudoku [Board]
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
    /// Returns an iterator that yields each index in this [SubBox]
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
    /// Defines the conversion [From] [Index] [Into] [SubBox]
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
