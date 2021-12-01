use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

pub trait Num:
    Add<Output = Self>
    + AddAssign
    + Copy
    + Clone
    + Debug
    + Display
    + Default
    + Div<Output = Self>
    + DivAssign
    + From<u8>
    + FromStr
    + Hash
    + Mul<Output = Self>
    + MulAssign
    + PartialEq
    + PartialOrd
    + Product
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
    + Sum
{
    fn abs(self) -> Self {
        if self >= Self::default() {
            self
        } else {
            self.unsafe_negate()
        }
    }

    /// Like `%` or `.rem()` except it always returns a non-negative result even
    /// if one or both of the inputs are negative.
    fn modulus(self, rhs: Self) -> Self {
        let abs_rhs = rhs.abs();
        ((self % abs_rhs) + abs_rhs) % abs_rhs
    }

    /// Negates the number if the number is signed, overflowing otherwise. This may
    /// be safely called on a type `T` if we have observed a negative value of type
    /// `T`.
    fn unsafe_negate(self) -> Self {
        Self::default() - self
    }
}

impl<T> Num for T where
    T: Add<Output = Self>
        + AddAssign
        + Copy
        + Clone
        + Debug
        + Display
        + Default
        + Div<Output = Self>
        + DivAssign
        + From<u8>
        + FromStr
        + Hash
        + Mul<Output = Self>
        + MulAssign
        + PartialEq
        + PartialOrd
        + Product
        + Rem<Output = Self>
        + RemAssign
        + Sub<Output = Self>
        + SubAssign
        + Sum
{
}

pub trait Signed: Num + Neg {}

impl<T> Signed for T where T: Num + Neg {}

pub trait Int: Num + Eq + Ord {}

impl<T> Int for T where T: Num + Eq + Ord {}
