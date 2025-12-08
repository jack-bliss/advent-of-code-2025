use std::fmt::Display;

#[derive(Debug)]
pub enum Operator {
    Sum,
    Mult,
}

impl Operator {
    pub fn symbol(&self) -> &str {
        match self {
            Self::Mult => "*",
            Self::Sum => "+",
        }
    }
}

#[derive(Debug)]
pub struct Equation {
    pub values: Vec<usize>,
    pub operator: Operator,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values_as_strings: Vec<String> = self.values.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", values_as_strings.join(self.operator.symbol()))
    }
}

impl Equation {
    pub fn solve(&self) -> usize {
        match self.operator {
            Operator::Sum => self.values.iter().sum(),
            Operator::Mult => self.values.iter().product(),
        }
    }
}
