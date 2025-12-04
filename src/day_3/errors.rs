use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParseBankError {
    NotEnoughBatteries { original: String, required: usize },
}

impl Display for ParseBankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseBankError::NotEnoughBatteries { original, required } => write!(
                f,
                "Not enough batteries in bank string '{}' (minimum required is {})",
                original, required
            ),
        }
    }
}

#[derive(Debug)]
pub enum GetMaxJoltageInRangeError {
    NoBatteryInRange { from: usize, to: usize },
}

impl Display for GetMaxJoltageInRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetMaxJoltageInRangeError::NoBatteryInRange { from, to } => write!(
                f,
                "No battery found in range from index {} to index {}",
                from, to
            ),
        }
    }
}

#[derive(Debug)]
pub enum GetMaxJoltageError {
    NotEnoughBatteriesEnabled {
        batteries: String,
        enabled: usize,
        min: usize,
    },
    TooManyBatteriesEnabled {
        batteries: String,
        enabled: usize,
        max: usize,
    },
}

impl Display for GetMaxJoltageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetMaxJoltageError::NotEnoughBatteriesEnabled {
                batteries,
                enabled,
                min,
            } => write!(
                f,
                "Not enough batteries enabled ({} enabled, min {} required) for bank {}",
                enabled, min, batteries
            ),
            GetMaxJoltageError::TooManyBatteriesEnabled {
                batteries,
                enabled,
                max,
            } => write!(
                f,
                "Too many batteries enabled ({} enabled, max {} allowed) for bank {}",
                enabled, max, batteries
            ),
        }
    }
}
