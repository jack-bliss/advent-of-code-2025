use std::fs;

pub mod range;

use range::Range;

pub fn sum_invalid_ids() -> Result<i64, std::io::Error> {
    let total = fs::read_to_string("./src/day_2/input.txt")
        .unwrap()
        .split(',')
        .filter_map(|range| match Range::try_from(range) {
            Ok(r) => Some(r.find_invalid_ids()),
            Err(e) => {
                eprintln!("Skipping invalid range: {}", e);
                None
            }
        })
        .flatten()
        .sum();

    Ok(total)
}
