use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("invalid row_matrix length. expected: {expected:?} found: {found:?}")]
    InvalidLength { expected: u32, found: u32 },
    #[error("cannot calculate {x_row:?} X {x_column:?} and {y_row:?} X {y_column:?}")]
    CannotCalculate {
        x_row: u32,
        x_column: u32,
        y_row: u32,
        y_column: u32,
    },
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