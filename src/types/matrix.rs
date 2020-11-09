#![allow(dead_code)]
use super::error::MatrixError;
use std::vec::Vec;

pub struct Matrix {
    matrix: Vec<Vec<f32>>,
}

impl Matrix {
    fn create(row: usize, column: usize, row_matrix: Vec<f32>) -> Result<Self, MatrixError> {
        let length = row * column;
        if length != row_matrix.len() {
            return Err(MatrixError::InvalidLength {
                expected: length,
                found: row_matrix.len(),
            });
        }

        let mut matrix = vec![];
        for m in 0..row {
            matrix.push(vec![]);
            for n in 0..column {
                matrix[m].push(row_matrix[m * column + n]);
            }
        }

        Ok(Matrix { matrix })
    }

    fn get_row(&self) -> usize {
        self.matrix.len()
    }

    fn get_column(&self) -> usize {
        self.matrix[0].len()
    }

    fn get_value(&self, row: usize, column: usize) -> f32 {
        self.matrix[row - 1][column - 1]
    }

    fn add(&self, y: &Self) -> Result<Self, MatrixError> {
        let row = self.get_row();
        let column = self.get_column();
        if row != y.get_row() || column != y.get_column() {
            return Err(MatrixError::CannotCalculate {
                x_row: row,
                x_column: column,
                y_row: y.get_row(),
                y_column: y.get_column(),
            });
        }

        let mut matrix = vec![];

        for m in 0..row {
            matrix.push(vec![]);
            for n in 0..column {
                matrix[m].push(self.get_value(m + 1, n + 1) + y.get_value(m + 1, n + 1));
            }
        }

        Ok(Matrix { matrix })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::error::MatrixError;
    #[test]
    fn create_matrix() {
        let matrix = Matrix::create(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(matrix.get_row(), 2);
        assert_eq!(matrix.get_column(), 3);
        assert_eq!(matrix.get_value(2, 1), 4.0);
    }

    #[test]
    fn invalid_length() {
        let err = match Matrix::create(1, 1, vec![]) {
            Ok(_) => panic!("wtf"),
            Err(err) => err,
        };
        let message = format!(
            "{}",
            MatrixError::InvalidLength {
                expected: 1,
                found: 0,
            }
        );
        assert_eq!(message, format!("{}", err));
    }

    #[test]
    fn add_two_matrix() {
        let x_matrix = Matrix::create(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let y_matrix = Matrix::create(2, 3, vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0]).unwrap();
        let new_matrix = x_matrix.add(&y_matrix).unwrap();

        assert_eq!(new_matrix.get_value(1, 1), 3.0);
        assert_eq!(new_matrix.get_value(1, 2), 5.0);
        assert_eq!(new_matrix.get_value(1, 3), 7.0);
        assert_eq!(new_matrix.get_value(2, 1), 9.0);
        assert_eq!(new_matrix.get_value(2, 2), 11.0);
        assert_eq!(new_matrix.get_value(2, 3), 13.0);
    }
}
