use crate::day_4::count_rolls_touching_at_most;

#[allow(dead_code)]
mod day_1;
#[allow(dead_code)]
mod day_2;
#[allow(dead_code)]
mod day_3;
#[allow(dead_code)]
mod day_4;

fn main() {
    match count_rolls_touching_at_most() {
        Ok(total) => println!("{}", total),
        Err(error) => println!("File read error: {}", error),
    }
}
