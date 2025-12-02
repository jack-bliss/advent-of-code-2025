use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum RangeParseError {
    Format {
        original: String,
    },
    Start {
        original: String,
        error: ParseIntError,
    },
    End {
        original: String,
        error: ParseIntError,
    },
    Range {
        original: String,
        start: i64,
        end: i64,
    },
}

impl Display for RangeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeParseError::Format { original } => {
                write!(f, "Invalid range format: '{}'", original)
            }
            RangeParseError::Start { original, error } => {
                write!(f, "Invalid start in range '{}': {}", original, error)
            }
            RangeParseError::End { original, error } => {
                write!(f, "Invalid end in range '{}': {}", original, error)
            }
            RangeParseError::Range {
                original,
                start,
                end,
            } => {
                write!(
                    f,
                    "Invalid range '{}': start ({}) is greater than end ({})",
                    original, start, end
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

fn is_repeated_n_times(str: &str, substr: &str, n: usize) -> bool {
    substr.repeat(n) == str
}

fn is_only_repeated_substring(str: &str) -> bool {
    for split_point in 1..(str.len() / 2) + 1 {
        if !str.len().is_multiple_of(split_point) {
            continue;
        }
        let (substr, _) = str.split_at(split_point);
        if is_repeated_n_times(str, substr, str.len() / split_point) {
            return true;
        }
    }
    false
}

fn id_is_invalid(id: &i64) -> bool {
    is_only_repeated_substring(&id.to_string())
}

impl Range {
    pub fn find_invalid_ids(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();

        for id in self.start..(self.end + 1) {
            if id_is_invalid(&id) {
                invalid_ids.push(id);
            }
        }

        invalid_ids
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl TryFrom<&str> for Range {
    type Error = RangeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if parts.len() != 2 {
            return Err(RangeParseError::Format {
                original: value.into(),
            });
        }

        let start = parts[0]
            .parse::<i64>()
            .map_err(|err| RangeParseError::Start {
                original: parts[0].into(),
                error: err,
            })?;

        let end = parts[1]
            .parse::<i64>()
            .map_err(|err| RangeParseError::End {
                original: parts[1].into(),
                error: err,
            })?;
        if start > end {
            return Err(RangeParseError::Range {
                original: value.into(),
                start,
                end,
            });
        }

        Ok(Range { start, end })
    }
}

#[cfg(test)]
mod tests {

    use super::{Range, id_is_invalid, is_repeated_n_times};

    #[test]
    fn test_is_repeated_n_times() {
        assert!(is_repeated_n_times("123123", "123", 2));
        assert!(is_repeated_n_times("121212121212", "12", 6));
        assert!(is_repeated_n_times("121212121212", "1212", 3));
        assert!(is_repeated_n_times("121212121212", "121212", 2));
        assert!(!is_repeated_n_times("555", "5", 2));
    }

    #[test]
    fn test_is_invalid_id() {
        assert!(id_is_invalid(&1010));
        assert!(id_is_invalid(&5555));
        assert!(id_is_invalid(&555));
        assert!(id_is_invalid(&232323));
        assert!(id_is_invalid(&121212121212));
        assert!(id_is_invalid(&123412341234));
        assert!(!id_is_invalid(&12120));
        assert!(!id_is_invalid(&1221));
    }

    #[test]
    fn test_range() {
        let range = Range::try_from("1000-1020").unwrap();
        let invalid_ids = range.find_invalid_ids();
        assert_eq!(invalid_ids, vec![1010]);
    }

    #[test]
    fn test_ranges_from_aoc() {
        let ranges: Vec<Vec<i64>> = [
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ]
        .iter()
        .map(|s| Range::try_from(*s).unwrap().find_invalid_ids())
        .collect();

        assert_eq!(
            ranges,
            vec![
                vec![11, 22],
                vec![99, 111],
                vec![999, 1010],
                vec![1188511885],
                vec![222222],
                vec![],
                vec![446446],
                vec![38593859],
                vec![565656],
                vec![824824824],
                vec![2121212121],
            ]
        );
    }
}
