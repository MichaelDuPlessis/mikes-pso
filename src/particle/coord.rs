use std::ops;

/// This trait is implemented for types that can be used to make up a particles coordinate
pub trait CoordinateElement: ops::Add + ops::AddAssign + Sized {}
