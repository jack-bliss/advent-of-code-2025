// mod day_1;
mod day_2;

fn main() {
    match day_2::sum_invalid_ids() {
        Ok(total) => println!("Sum of Invalid IDs: {}", total),
        Err(e) => eprintln!("Error processing invalid IDs: {}", e),
    }
}
