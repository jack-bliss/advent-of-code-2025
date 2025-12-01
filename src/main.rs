mod day_1;
use day_1::apply_turns;

fn main() {
    match apply_turns() {
        Ok(result) => {
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
