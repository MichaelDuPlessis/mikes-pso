use std::ops;

/// This trait is implemented for types that can be used to make up a particles coordinate
// TODO: Remove Copy trait bound at a later date, only in use now for first implementation
pub trait Coordinate: ops::Add + ops::AddAssign + Sized + Copy {}
