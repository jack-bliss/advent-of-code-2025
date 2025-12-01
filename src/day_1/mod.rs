mod dial;
mod turn;

use std::{fmt::Display, fs};

use dial::Dial;
use turn::Turn;

pub struct TurnApplicationResult {
    pub zero_count: i32,
    pub total_crossings: i32,
}

impl TurnApplicationResult {
    pub fn empty() -> Self {
        Self {
            zero_count: 0,
            total_crossings: 0,
        }
    }
}

impl Display for TurnApplicationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Times Hit Zero: {}\nTotal Crossings: {}",
            self.zero_count, self.total_crossings
        )
    }
}

const TURNS_FILE_PATH: &str = "./src/day_1/turns.txt";

pub fn apply_turns() -> Result<TurnApplicationResult, std::io::Error> {
    let mut dial = Dial::new();

    let result: TurnApplicationResult = fs::read_to_string(TURNS_FILE_PATH)?
        .lines()
        .map(Turn::try_from)
        .filter_map(|turn| match turn {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("Skipping invalid turn: {}", e);
                None
            }
        })
        .fold(TurnApplicationResult::empty(), |result, turn| {
            let crossings = dial.turn(turn);
            TurnApplicationResult {
                total_crossings: result.total_crossings + crossings,
                zero_count: result.zero_count + dial.zero_contribution(),
            }
        });

    Ok(result)
}
