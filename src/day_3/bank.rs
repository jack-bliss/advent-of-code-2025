use super::errors::{GetMaxJoltageError, GetMaxJoltageInRangeError, ParseBankError};
use std::fmt::Display;

#[derive(Debug)]
pub struct Bank {
    batteries: Vec<usize>,
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let batteries_string: String = self
            .batteries
            .iter()
            .fold("".to_string(), |string, battery| {
                string + &battery.to_string()
            });
        write!(f, "B({})", batteries_string)
    }
}

impl TryFrom<&str> for Bank {
    type Error = ParseBankError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let batteries: Vec<usize> = value
            .chars()
            .filter_map(|char| match char.to_digit(10) {
                Some(value) => value.try_into().ok(),
                None => None,
            })
            .collect();

        match batteries.len() {
            0..2 => Err(ParseBankError::NotEnoughBatteries {
                original: value.into(),
                required: 2,
            }),
            _ => Ok(Bank { batteries }),
        }
    }
}

impl Bank {
    pub fn get_max_joltage_in_range(
        &self,
        from_index: usize,
        to_index: usize,
    ) -> Result<(usize, &usize), GetMaxJoltageInRangeError> {
        let result = self
            .batteries
            .iter()
            .skip(from_index)
            .take(to_index - from_index)
            .enumerate()
            .max_by(|a, b| match a.1.cmp(b.1) {
                std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                other => other,
            })
            .ok_or(GetMaxJoltageInRangeError::NoBatteryInRange {
                from: from_index,
                to: to_index,
            })?;
        Ok(result)
    }

    pub fn get_max_joltage(&self, enabled: usize) -> Result<usize, GetMaxJoltageError> {
        if enabled < 2 {
            return Err(GetMaxJoltageError::NotEnoughBatteriesEnabled {
                batteries: self.to_string(),
                enabled,
                min: 2,
            });
        }
        if enabled > self.batteries.len() {
            return Err(GetMaxJoltageError::TooManyBatteriesEnabled {
                batteries: self.to_string(),
                enabled,
                max: self.batteries.len(),
            });
        }
        let mut from_index: usize = 0;
        let mut to_index = self.batteries.len() - (enabled - 1);
        let mut joltage: usize = 0;
        for battery in 0..enabled {
            match self.get_max_joltage_in_range(from_index, to_index) {
                Ok((index, value)) => {
                    // jump forward to next range
                    from_index += index + 1;
                    // lets us get one closer to the end each time
                    to_index += 1;
                    // add the joltage contribution to the running total
                    joltage += value * 10_usize.pow((enabled - battery) as u32 - 1);
                }
                Err(err) => {
                    eprintln!(
                        "Failed to get max joltage for bank {} with {} batteries enabled: {:?}",
                        self, enabled, err
                    );
                    return Err(GetMaxJoltageError::TooManyBatteriesEnabled {
                        batteries: self.to_string(),
                        enabled,
                        max: self.batteries.len(),
                    });
                }
            }
        }
        Ok(joltage)
    }
}

#[cfg(test)]
mod tests {

    use crate::day_3::bank::ParseBankError;

    use super::Bank;

    #[test]
    fn test_parse_bank() {
        let bank = Bank::try_from("12345").unwrap();
        assert_eq!(bank.batteries, vec![1, 2, 3, 4, 5]);
        let bank = Bank::try_from("123abc3def").unwrap();
        assert_eq!(bank.batteries, vec![1, 2, 3, 3]);
        let bank = Bank::try_from("1a").unwrap_err();
        assert_eq!(
            bank,
            ParseBankError::NotEnoughBatteries {
                original: "1a".into(),
                required: 2,
            }
        )
    }

    #[test]
    fn test_max_joltage_2() {
        let bank = Bank::try_from("987654321111111").unwrap();
        assert_eq!(bank.get_max_joltage(2).unwrap(), 98);
        let bank = Bank::try_from("811111111111119").unwrap();
        assert_eq!(bank.get_max_joltage(2).unwrap(), 89);
        let bank = Bank::try_from("234234234234278").unwrap();
        assert_eq!(bank.get_max_joltage(2).unwrap(), 78);
        let bank = Bank::try_from("818181911112111").unwrap();
        assert_eq!(bank.get_max_joltage(2).unwrap(), 92);
    }

    #[test]
    fn test_max_joltage_12() {
        let bank = Bank::try_from("987654321111111").unwrap();
        assert_eq!(bank.get_max_joltage(12).unwrap(), 987654321111);
        let bank = Bank::try_from("811111111111119").unwrap();
        assert_eq!(bank.get_max_joltage(12).unwrap(), 811111111119);
        let bank = Bank::try_from("234234234234278").unwrap();
        assert_eq!(bank.get_max_joltage(12).unwrap(), 434234234278);
        let bank = Bank::try_from("818181911112111").unwrap();
        assert_eq!(bank.get_max_joltage(12).unwrap(), 888911112111);
    }
}
