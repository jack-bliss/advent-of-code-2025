use std::{fmt::Display, fs};

use crate::day_6::{
    equation::{Equation, Operator},
    matrix::Matrix,
};

pub mod equation;
pub mod matrix;

#[derive(Debug)]
pub enum ParseError {
    NoRows,
    IoError(std::io::Error),
    InvalidEquation(String),
}

pub fn parse_input(input: String) -> Result<Vec<Equation>, ParseError> {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let height = rows.len();
    let Some(width) = rows.first().map(|row| row.len()) else {
        return Err(ParseError::NoRows);
    };

    let data: Vec<&str> = rows
        .iter()
        .flat_map(|row| row.iter().map(|value| value.to_owned()))
        .collect();

    let matrix = Matrix::new(width, height, data);

    let mut equations: Vec<Equation> = Vec::new();

    for mut column in matrix.cols() {
        let last = column
            .pop()
            .ok_or(ParseError::InvalidEquation(String::from(
                "Couldn't get last entry for a column!",
            )))?;
        let operator = match last {
            "*" => Ok(Operator::Mult),
            "+" => Ok(Operator::Sum),
            op => Err(ParseError::InvalidEquation(format!(
                "Invalid operator '{op}'"
            ))),
        }?;
        let values: Result<Vec<usize>, _> =
            column.iter().map(|value| value.parse::<usize>()).collect();

        let values = values
            .map_err(|error| ParseError::InvalidEquation(format!("Couldn't parse int: {error}")))?;

        equations.push(Equation { values, operator });
    }

    Ok(equations)
}

pub fn solve_equations() -> Result<usize, ParseError> {
    let input = fs::read_to_string("./src/day_6/input.txt").map_err(ParseError::IoError)?;
    let equations = parse_input(input)?;
    Ok(equations.iter().map(|eq| eq.solve()).sum::<usize>())
}

#[cfg(test)]
mod test {
    use super::parse_input;

    #[test]
    fn test_parse() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();
        let result = parse_input(input).expect("Should not throw");
        result.iter().for_each(|eq| {
            println!("{eq} = {}", eq.solve());
        });
        assert_eq!(result.iter().map(|eq| eq.solve()).sum::<usize>(), 4277556);
    }
}
