use crate::day_1::turn::Turn;
use std::fmt::Display;

// a struct for storing the state of the dial
#[derive(Debug)]
pub struct Dial {
    pub value: i32,
}

impl Dial {
    // dials start at 50 by default
    pub const fn new() -> Self {
        Self { value: 50 }
    }

    // used to count the total number of times we land on exactly zero
    pub fn zero_contribution(&self) -> i32 {
        if self.value == 0 { 1 } else { 0 }
    }
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D@{}", self.value)
    }
}

// turning the dial mutates the state and returns the number of times we crossed zero
impl Dial {
    pub fn turn(&mut self, turn: Turn) -> i32 {
        // keep track of what we had at the start
        let start = self.value;

        // simplify things a bit first
        // determine how many full rotations we have
        let turn_size = turn.abs();
        let full_rotations = turn_size / 100;

        // each full rotation = one zero crossing
        let mut zero_crossings = full_rotations;

        // only turn the remainder
        let adjustment = turn_size % 100;
        self.value += match turn {
            Turn::Left(_) => -adjustment,
            Turn::Right(_) => adjustment,
        };

        // case where we land exactly on zero from another number
        if self.value == 0 && start != 0 {
            zero_crossings += 1;
        } else if self.value < 0 {
            // case where we wrap below 0, not starting from 0
            if start != 0 {
                zero_crossings += 1;
            }
            // adjust value to wrap around
            self.value += 100;
        } else if self.value >= 100 {
            // case where we wrap above 99
            zero_crossings += 1;
            // adjust value to wrap around
            self.value -= 100;
        }

        // return the number of zero crossings we found
        zero_crossings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_1::turn::Turn;

    #[test]
    fn test_dial_turning_left_to_zero() {
        let mut dial = Dial::new();
        let zero_count = dial.turn(Turn::Left(50));
        assert_eq!(dial.value, 0);
        assert_eq!(zero_count, 1);
    }

    #[test]
    fn test_dial_turning_left_past_zero() {
        let mut dial = Dial::new();
        let zero_count = dial.turn(Turn::Left(51));
        assert_eq!(dial.value, 99);
        assert_eq!(zero_count, 1);
    }

    #[test]
    fn test_dial_turning_left_way_past_zero() {
        let mut dial = Dial::new();
        let zero_count = dial.turn(Turn::Left(950));
        assert_eq!(dial.value, 0);
        assert_eq!(zero_count, 10);
    }

    #[test]
    fn test_dial_turning_right_past_hundred() {
        let mut dial = Dial::new();
        let zero_count = dial.turn(Turn::Right(60));
        assert_eq!(dial.value, 10);
        assert_eq!(zero_count, 1);
    }

    #[test]
    fn test_dial_turning_right_way_past_hundred() {
        let mut dial = Dial::new();
        let zero_count = dial.turn(Turn::Right(1000));
        assert_eq!(dial.value, 50);
        assert_eq!(zero_count, 10);
    }

    #[test]
    fn test_dial_turns() {
        let mut dial = Dial::new();
        let turns = vec![Turn::Left(60), Turn::Right(130), Turn::Right(180)];
        let mut zero_count = 0;
        for turn in turns {
            zero_count += dial.turn(turn);
        }
        assert_eq!(dial.value, 0);
        assert_eq!(zero_count, 5);
    }

    #[test]
    fn test_dial_turns_two() {
        let mut dial = Dial::new();
        let turns = vec![Turn::Right(50), Turn::Right(100), Turn::Right(50)];
        let mut zero_count = 0;
        for turn in turns {
            zero_count += dial.turn(turn);
        }
        assert_eq!(dial.value, 50);
        assert_eq!(zero_count, 2);
    }

    #[test]
    fn example_from_aoc() {
        let mut dial = Dial::new();
        let zero_crossings = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .iter()
        .map(|s| Turn::try_from(*s).unwrap())
        .fold(0, |total, turn| total + dial.turn(turn));
        assert_eq!(zero_crossings, 6);
    }
}
