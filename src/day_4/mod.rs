use std::fs;

use crate::day_4::paper_rolls::{PaperRollRemover, PaperRolls};

pub mod paper_rolls;

pub fn count_rolls_touching_at_most() -> Result<u32, std::io::Error> {
    let input = fs::read_to_string("./src/day_4/input.txt")?;
    let rolls = PaperRolls::from(input);
    let mut paper_roll_remover = PaperRollRemover {
        rolls,
        touching_at_most: 3,
    };
    let total_removed = paper_roll_remover.remove_all();
    fs::write(
        "./src/day_4/output.txt",
        paper_roll_remover.rolls.to_string(),
    )?;
    Ok(total_removed)
}
