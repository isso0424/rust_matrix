pub trait MatrixNumber
where
    Self: num_traits::Signed + std::marker::Copy,
{
}

impl MatrixNumber for f32 {}

impl MatrixNumber for f64 {}

impl MatrixNumber for i8 {}

impl MatrixNumber for i16 {}

impl MatrixNumber for i32 {}

impl MatrixNumber for i64 {}

impl MatrixNumber for i128 {}
