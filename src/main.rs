use crate::day_3::find_total_max_joltage;

// mod day_1;
// mod day_2;
mod day_3;
fn main() {
    match find_total_max_joltage(12) {
        Ok(total) => println!("Total max joltage: {total}"),
        Err(error) => eprintln!("{:?}", error),
    }
}
