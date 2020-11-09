#![allow(dead_code)]
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MatrixError {
    #[error("invalid row_matrix length. expected: {expected:?} found: {found:?}")]
    InvalidLength { expected: usize, found: usize },
    #[error("cannot calculate {x_row:?} X {x_column:?} and {y_row:?} X {y_column:?}")]
    CannotCalculate {
        x_row: usize,
        x_column: usize,
        y_row: usize,
        y_column: usize,
    },
    #[error("non-supported matrix shape in this function. {row:?} X {column:?}")]
    NonSupportedMatrixShape { row: usize, column: usize },
    #[error("determinant is zero.")]
    ZeroDeterminant,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_invalid_length() {
        let invalid_length = MatrixError::InvalidLength {
            expected: 3,
            found: 4,
        };
        let message = format!("{}", invalid_length);
        assert_eq!(message, "invalid row_matrix length. expected: 3 found: 4");
    }

    #[test]
    fn check_cannot_calculate() {
        let cannot_calculate = MatrixError::CannotCalculate {
            x_row: 3,
            x_column: 4,
            y_row: 5,
            y_column: 6,
        };
        let message = format!("{}", cannot_calculate);
        assert_eq!(message, "cannot calculate 3 X 4 and 5 X 6");
    }
}
