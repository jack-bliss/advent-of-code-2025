use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

type Coord = (usize, usize);

#[derive(Debug, Default, PartialEq)]
pub struct Matrix<T> {
    size: Coord,
    data: Vec<T>,
}

pub enum MatrixIteratorDirection {
    Rows,
    Cols,
}

impl MatrixIteratorDirection {
    pub fn orient<T>(&self, row_wise: T, col_wise: T) -> (T, T) {
        match self {
            MatrixIteratorDirection::Rows => (row_wise, col_wise),
            MatrixIteratorDirection::Cols => (col_wise, row_wise),
        }
    }
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    direction: MatrixIteratorDirection,
    current_index: usize,
}

impl<'a, T> MatrixIterator<'a, T> {
    pub fn over_rows(matrix: &'a Matrix<T>) -> Self {
        Self {
            matrix,
            direction: MatrixIteratorDirection::Rows,
            current_index: 0,
        }
    }
    pub fn over_cols(matrix: &'a Matrix<T>) -> Self {
        Self {
            matrix,
            direction: MatrixIteratorDirection::Cols,
            current_index: 0,
        }
    }
}

impl<'a, T: Default + Copy> Iterator for MatrixIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (vector_length, iteration_count) = self
            .direction
            .orient(self.matrix.width(), self.matrix.height());

        if self.current_index >= iteration_count {
            return None;
        }
        let range = 0..vector_length;
        let current_iteration_index = self.current_index;
        let values = range.map(|index_along_vector| {
            self.matrix[self
                .direction
                .orient(index_along_vector, current_iteration_index)]
        });
        self.current_index += 1;
        Some(values.collect())
    }
}

impl<T: Default + Copy> Matrix<T> {
    fn zeros(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            data: vec![T::default(); width * height],
        }
    }

    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            size: (width, height),
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn rows(&self) -> MatrixIterator<'_, T> {
        MatrixIterator::over_rows(self)
    }

    pub fn cols(&self) -> MatrixIterator<'_, T> {
        MatrixIterator::over_cols(self)
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeros(self.height(), self.width());
        for y in 0..self.height() {
            for x in 0..self.width() {
                result[(x, y)] = self[(y, x)];
            }
        }
        result
    }
}

impl<T> Index<Coord> for Matrix<T> {
    type Output = T;
    fn index(&self, (x, y): Coord) -> &Self::Output {
        let (width, _) = self.size;
        &self.data[y * width + x]
    }
}

impl<T> IndexMut<Coord> for Matrix<T> {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Self::Output {
        let (width, _) = self.size;
        &mut self.data[y * width + x]
    }
}

impl<T: Display + Default + Copy> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .rows()
            .map(|row| {
                row.iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{lines}")
    }
}

#[cfg(test)]
mod test {
    use crate::day_6::matrix::Matrix;

    const TEST_DATA: [&str; 16] = [
        "123", "328", "51", "64", //
        "45", "64", "387", "23", //
        "6", "98", "215", "314", //
        "*", "+", "*", "+", //
    ];

    #[test]
    fn test_zeroes() {
        let matrix: Matrix<u32> = Matrix::zeros(5, 3);
        println!("{matrix}");
    }

    #[test]
    fn test_indexing() {
        let mut matrix: Matrix<u32> = Matrix::zeros(5, 3);
        matrix[(2, 0)] = 1;
        matrix[(3, 1)] = 2;
        matrix[(1, 2)] = 3;
        assert_eq!(matrix[(2, 0)], 1);
    }

    #[test]
    fn test_aoc_example() {
        let matrix = Matrix::new(4, 4, TEST_DATA.to_vec());

        println!("{matrix}");
        println!("{}", matrix.transpose());
    }

    #[test]
    fn test_row_iterators() {
        let mut matrix = Matrix::new(4, 4, TEST_DATA.to_vec());

        let mut rows = matrix.rows();

        assert_eq!(rows.next(), Some(vec!["123", "328", "51", "64"]));
        assert_eq!(rows.next(), Some(vec!["45", "64", "387", "23"]));
        assert_eq!(rows.next(), Some(vec!["6", "98", "215", "314"]));
        assert_eq!(rows.next(), Some(vec!["*", "+", "*", "+"]));
        assert_eq!(rows.next(), None);

        matrix[(0, 0)] = "321";
    }

    #[test]
    fn test_row_iterators_2() {
        let matrix = Matrix::new(4, 4, TEST_DATA.to_vec());

        assert_eq!(matrix.rows().next(), Some(vec!["123", "328", "51", "64"]));
    }

    #[test]
    fn test_col_iterators() {
        let matrix = Matrix::new(4, 4, TEST_DATA.to_vec());

        let mut cols = matrix.cols();

        assert_eq!(cols.next(), Some(vec!["123", "45", "6", "*"]));
        assert_eq!(cols.next(), Some(vec!["328", "64", "98", "+"]));
        assert_eq!(cols.next(), Some(vec!["51", "387", "215", "*"]));
        assert_eq!(cols.next(), Some(vec!["64", "23", "314", "+"]));
        assert_eq!(cols.next(), None);
    }
}
