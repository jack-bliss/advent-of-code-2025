use std::fs;
pub mod bank;
pub mod errors;

use bank::Bank;

pub fn find_total_max_joltage(batteries_enabled: usize) -> Result<usize, std::io::Error> {
    let total = fs::read_to_string("./src/day_3/input.txt")?
        .lines()
        .filter_map(|line| {
            let bank = match Bank::try_from(line) {
                Ok(bank) => bank,
                Err(error) => {
                    eprintln!("{:?}", error);
                    return None;
                }
            };
            match bank.get_max_joltage(batteries_enabled) {
                Ok(joltage) => Some(joltage),
                Err(error) => {
                    eprintln!("{:?}", error);
                    None
                }
            }
        })
        .sum();
    Ok(total)
}
