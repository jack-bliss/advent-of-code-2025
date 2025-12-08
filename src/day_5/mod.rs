use std::{fmt::Display, fs};

use crate::day_5::ingredients::{Ingredients, ParseIngredientsError};

mod ingredients;

#[derive(Debug)]
pub enum CountFreshError {
    Io(std::io::Error),
    Parse(ParseIngredientsError),
}

impl Display for CountFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO Error: {error}"),
            Self::Parse(error) => write!(f, "Parse Error: {error}"),
        }
    }
}

pub fn count_fresh_ingredients() -> Result<(usize, usize), CountFreshError> {
    let input = fs::read_to_string("./src/day_5/input.txt").map_err(CountFreshError::Io)?;
    let ingredients = Ingredients::try_from(input).map_err(CountFreshError::Parse)?;
    Ok((
        ingredients.get_fresh().len(),
        ingredients.count_considered_fresh(),
    ))
}
