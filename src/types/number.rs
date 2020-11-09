pub trait MatrixNumber
where
    Self: std::ops::Add<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Div<Output = Self>
        + std::cmp::PartialEq
        + std::marker::Sized
        + std::marker::Copy,
{
    fn zero() -> Self;

    fn one() -> Self;

    fn minus_one() -> Self;
}

impl MatrixNumber for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn minus_one() -> Self {
        -1.0
    }
}

impl MatrixNumber for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn minus_one() -> Self {
        -1.0
    }
}

impl MatrixNumber for i8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn minus_one() -> Self {
        -1
    }
}

impl MatrixNumber for i16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn minus_one() -> Self {
        -1
    }
}

impl MatrixNumber for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn minus_one() -> Self {
        -1
    }
}

impl MatrixNumber for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn minus_one() -> Self {
        -1
    }
}

impl MatrixNumber for i128 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn minus_one() -> Self {
        -1
    }
}
