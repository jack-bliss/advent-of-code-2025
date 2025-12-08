use std::fmt::Display;

use crate::day_2::range::RangeParseError;

use super::super::day_2::range::Range;

pub struct Ingredients {
    fresh_ranges: Vec<Range>,
    available: Vec<usize>,
}

impl Ingredients {
    pub fn get_fresh(&self) -> Vec<&usize> {
        self.available
            .iter()
            .filter(|id| {
                let id = **id as i64;
                self.fresh_ranges
                    .iter()
                    .any(|range| range.start <= id && id <= range.end)
            })
            .collect()
    }
    pub fn merge_ranges(&self) -> Vec<Range> {
        // first, sort in descending order, so we can treat the vec like a stack
        let mut sorted_ranges = self.fresh_ranges.clone();
        sorted_ranges.sort_by(|a, b| b.start.cmp(&a.start));
        // set up the return vector
        let mut merged_ranges = vec![];
        // if the list is empty, do an early return
        // most convenient time to do this check because we can use let Some
        let Some(mut current) = sorted_ranges.pop() else {
            return merged_ranges;
        };
        // while there is still a range left in the stack...
        while let Some(next) = sorted_ranges.pop() {
            // if the ranges overlap
            if (current.start <= next.start && next.start <= current.end)
                // or if the ranges are touching
                || (current.end + 1 == next.start)
            {
                // update the current range's end as necessary
                current.end = current.end.max(next.end);
            } else {
                // otherwise, the current range doesn't need any more merging
                merged_ranges.push(current);
                // and we can make this new range the current range
                current = next;
            }
        }
        // remember to add the current range to the return vector
        merged_ranges.push(current);
        merged_ranges
    }
    pub fn count_considered_fresh(&self) -> usize {
        let merged_ranges = self.merge_ranges();
        merged_ranges
            .iter()
            .map(|range| {
                println!("{range}");
                let Range { start, end } = range;
                (1 + end - start) as usize
            })
            .sum()
    }
}

#[derive(Debug)]
pub enum ParseIngredientsError {
    InvalidRange(RangeParseError),
    NoGap(String),
    InvalidId(String),
}

impl Display for ParseIngredientsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange(message) => {
                write!(f, "ParseIngredientsError::InvalidRange({message})")
            }
            Self::NoGap(message) => write!(f, "ParseIngredientsError::NoGap({message})"),
            Self::InvalidId(message) => write!(f, "ParseIngredientsError::InvalidId({message})"),
        }
    }
}

impl TryFrom<String> for Ingredients {
    type Error = ParseIngredientsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut is_ranges = true;
        let mut fresh_ranges: Vec<Range> = Vec::new();
        let mut available: Vec<usize> = Vec::new();
        for line in value.lines() {
            if line.is_empty() {
                is_ranges = false;
                continue;
            }
            if is_ranges {
                match Range::try_from(line) {
                    Ok(range) => fresh_ranges.push(range),
                    Err(error) => {
                        return Err(ParseIngredientsError::InvalidRange(error));
                    }
                }
            } else {
                match line.parse::<usize>() {
                    Ok(value) => available.push(value),
                    Err(error) => return Err(ParseIngredientsError::InvalidId(format!("{error}"))),
                }
            }
        }
        if is_ranges {
            return Err(ParseIngredientsError::NoGap(String::from(
                "There was no empty line in the input.",
            )));
        }
        Ok(Self {
            fresh_ranges,
            available,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::day_2::range::Range;

    use super::Ingredients;

    const TEST_INPUT: &str = "3-5
10-14
15-20

1
5
8
11
17
32";

    #[test]
    fn test_parse() {
        let ingredients =
            Ingredients::try_from(TEST_INPUT.to_string()).expect("Should parse correctly");
        assert_eq!(ingredients.fresh_ranges.len(), 4);
        assert_eq!(ingredients.available.len(), 6);
    }

    #[test]
    fn test_get_fresh() {
        let ingredients =
            Ingredients::try_from(TEST_INPUT.to_string()).expect("Should parse correctly");
        assert_eq!(ingredients.get_fresh(), vec![&5, &11, &17]);
    }

    #[test]
    fn test_merge() {
        let ingredients =
            Ingredients::try_from(TEST_INPUT.to_string()).expect("Should parse correctly");
        assert_eq!(
            ingredients.merge_ranges(),
            vec![Range::of(3, 5), Range::of(10, 20)]
        );
    }
    #[test]
    fn test_count_considered_fresh() {
        let ingredients =
            Ingredients::try_from(TEST_INPUT.to_string()).expect("Should parse correctly");
        assert_eq!(ingredients.count_considered_fresh(), 14);
    }
}
