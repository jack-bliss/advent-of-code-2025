use std::fmt::Display;

type Row = Vec<u32>;
type Rows = Vec<Row>;

#[derive(Clone)]
pub struct PaperRolls {
    rows: Rows,
}

impl PaperRolls {
    pub fn new(rows: Rows) -> Self {
        PaperRolls { rows }
    }
}

impl Display for PaperRolls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if *cell == 1 { '@' } else { '.' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{output}")
    }
}

impl From<String> for PaperRolls {
    fn from(value: String) -> Self {
        Self::new(
            value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| if char == '@' { 1 } else { 0 })
                        .collect()
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
pub struct Location {
    x: i32,
    y: i32,
}

impl std::ops::Add<&Location> for &Location {
    type Output = Location;

    fn add(self, rhs: &Location) -> Self::Output {
        Location::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Location {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub const fn l(x: i32, y: i32) -> Location {
    Location::new(x, y)
}

const OFFSETS: [Location; 8] = [
    // above
    l(-1, -1),
    l(0, -1),
    l(1, -1),
    // sides
    l(-1, 0),
    l(1, 0),
    // below
    l(-1, 1),
    l(0, 1),
    l(1, 1),
];

impl PaperRolls {
    fn get_cell_value(&self, loc: &Location) -> u32 {
        let Location { x, y } = loc;
        if *y < 0 || *x < 0 {
            return 0;
        }
        let Some(row) = self.rows.get(*y as usize) else {
            return 0;
        };
        let Some(cell) = row.get(*x as usize) else {
            return 0;
        };
        *cell
    }

    fn set_cell_value(&mut self, loc: &Location, value: u32) {
        let Location { x, y } = loc;
        if *y < 0 || *x < 0 {
            return;
        }
        let Some(row) = self.rows.get(*y as usize) else {
            return;
        };
        let mut row = row.clone();
        row[*x as usize] = value;
        self.rows[*y as usize] = row;
    }

    fn touching(&self, loc: &Location) -> u32 {
        OFFSETS
            .iter()
            .map(|offset: &Location| self.get_cell_value(&(offset + loc)))
            .sum()
    }

    pub fn count_touching_at_most(&self, touching_at_most: u32) -> u32 {
        let mut touching = 0;
        for y in 0..self.rows.len() {
            let Some(row) = self.rows.get(y) else {
                continue;
            };
            for x in 0..row.len() {
                let cell = l(x as i32, y as i32);
                let cell_is_roll = self.get_cell_value(&cell) == 1;
                let cell_is_touching_at_most = self.touching(&cell) <= touching_at_most;
                touching += if cell_is_roll && cell_is_touching_at_most {
                    1
                } else {
                    0
                };
            }
        }
        touching
    }

    pub fn remove_touching_at_most(&mut self, touching_at_most: u32) -> u32 {
        let mut remove_at = vec![];
        for y in 0..self.rows.len() {
            let Some(row) = self.rows.get(y) else {
                continue;
            };
            for x in 0..row.len() {
                let cell = l(x as i32, y as i32);
                let cell_is_roll = self.get_cell_value(&cell) == 1;
                let cell_is_touching_at_most = self.touching(&cell) <= touching_at_most;
                if cell_is_roll && cell_is_touching_at_most {
                    remove_at.push(cell)
                }
            }
        }
        for cell in &remove_at {
            self.set_cell_value(cell, 0);
        }
        remove_at.len() as u32
    }
}

pub struct PaperRollRemover {
    pub rolls: PaperRolls,
    pub touching_at_most: u32,
}

impl PaperRollRemover {
    pub fn remove_all(&mut self) -> u32 {
        self.sum()
    }
}

impl Iterator for PaperRollRemover {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.rolls.remove_touching_at_most(self.touching_at_most);
        if next == 0 { None } else { Some(next) }
    }
}

#[cfg(test)]
pub mod test {

    use crate::day_4::paper_rolls::PaperRollRemover;

    use super::{PaperRolls, l};

    macro_rules! test_parse_paper_rolls {
        ($grid:literal, $rows:expr) => {{
            let rolls = PaperRolls::from($grid.to_string());
            assert_eq!(rolls.rows, $rows);
        }};
    }

    #[test]
    fn test_parse_small() {
        test_parse_paper_rolls!(
            "@..
...
...",
            vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        test_parse_paper_rolls!(
            "...
...
.@.",
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 0]]
        );
        test_parse_paper_rolls!(
            "....
.@..
...@
....",
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0]
            ]
        );
    }

    #[test]
    fn test_parse_example() {
        let rolls = PaperRolls::from(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );
        assert_eq!(rolls.rows.len(), 10);
        assert!(rolls.rows.iter().all(|row| row.len() == 10));
    }

    #[test]
    fn test_touching() {
        let rolls = PaperRolls::from(
            "...
...
..."
            .to_string(),
        );
        assert_eq!(rolls.touching(&l(1, 1)), 0);
        let rolls = PaperRolls::from(
            "@..
...
..."
            .to_string(),
        );
        assert_eq!(rolls.touching(&l(1, 1)), 1);
        let rolls = PaperRolls::from(
            "@..
...
..@"
            .to_string(),
        );
        assert_eq!(rolls.touching(&l(1, 1)), 2);

        let rolls = PaperRolls::from(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );
        assert_eq!(rolls.touching(&l(5, 2)), 6);
        assert_eq!(rolls.touching(&l(2, 5)), 6);

        assert_eq!(rolls.touching(&l(0, 0)), 2);
        assert_eq!(rolls.touching(&l(5, 0)), 3);
        assert_eq!(rolls.touching(&l(9, 0)), 3);

        assert_eq!(rolls.touching(&l(0, 5)), 4);
        assert_eq!(rolls.touching(&l(9, 5)), 4);

        assert_eq!(rolls.touching(&l(0, 9)), 1);
        assert_eq!(rolls.touching(&l(5, 9)), 5);
        assert_eq!(rolls.touching(&l(9, 9)), 2);
    }

    #[test]
    fn test_touching_at_most() {
        let rolls = PaperRolls::from(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );

        assert_eq!(rolls.count_touching_at_most(3), 13);
    }

    #[test]
    fn test_remove_touching_at_most() {
        let mut rolls = PaperRolls::from(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );
        // assert_eq!(rolls.remove_touching_at_most(3), 13);
        let removed = rolls.remove_touching_at_most(3);
        assert_eq!(removed, 13);
        assert_eq!(
            rolls.to_string(),
            ".......@..
.@@.@.@.@@
@@@@@...@@
@.@@@@..@.
.@.@@@@.@.
.@@@@@@@.@
.@.@.@.@@@
..@@@.@@@@
.@@@@@@@@.
....@@@..."
        );
        let removed = rolls.remove_touching_at_most(3);
        assert_eq!(removed, 12);
        assert_eq!(
            rolls.to_string(),
            "..........
.@@.....@.
.@@@@...@@
..@@@@....
.@.@@@@...
..@@@@@@..
...@.@.@@@
..@@@.@@@@
..@@@@@@@.
....@@@..."
        );
    }

    #[test]
    fn test_paper_roll_remover() {
        let mut remover = PaperRollRemover {
            rolls: PaperRolls::from(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                    .to_string(),
            ),
            touching_at_most: 3,
        };
        let total_removed = remover.remove_all();
        assert_eq!(total_removed, 43);
        assert_eq!(
            remover.rolls.to_string(),
            "..........
..........
..........
....@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@..."
        )
    }
}
