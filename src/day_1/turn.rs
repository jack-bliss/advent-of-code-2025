use std::fmt::Display;

// two different things can go wrong when parsing a turn
#[derive(Debug)]
pub enum TurnParseError {
    InvalidDirection {
        original: String,
        invalid: String,
    },
    InvalidNumber {
        original: String,
        invalid: String,
        error: std::num::ParseIntError,
    },
}

impl Display for TurnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnParseError::InvalidDirection { original, invalid } => {
                write!(
                    f,
                    "Failed to parse '{}'. Invalid direction '{}'",
                    original, invalid
                )
            }
            TurnParseError::InvalidNumber {
                original,
                invalid,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse '{}'. Invalid number '{}': {}",
                    original, invalid, error
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    Left(i32),
    Right(i32),
}

impl TryFrom<&str> for Turn {
    type Error = TurnParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, num_str) = value.split_at(1);
        let num: i32 = num_str
            .parse()
            .map_err(|err| TurnParseError::InvalidNumber {
                original: value.into(),
                invalid: num_str.into(),
                error: err,
            })?;
        match dir {
            "L" => Ok(Turn::Left(num)),
            "R" => Ok(Turn::Right(num)),
            other => Err(TurnParseError::InvalidDirection {
                original: value.into(),
                invalid: other.into(),
            }),
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::Left(n) => write!(f, "L{}", n),
            Turn::Right(n) => write!(f, "R{}", n),
        }
    }
}

impl Turn {
    pub fn abs(&self) -> i32 {
        match self {
            Turn::Left(n) | Turn::Right(n) => *n,
        }
    }
}
