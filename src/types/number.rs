pub trait MatrixNumber
where
    Self: std::ops::Add + std::ops::Mul + std::marker::Sized,
{
    fn zero() -> Self;

    fn one() -> Self;
}

impl MatrixNumber for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl MatrixNumber for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl MatrixNumber for i8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for i16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for i128 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for u16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl MatrixNumber for u128 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}
