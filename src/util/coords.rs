use crate::util::nums::Num;
use derive_more::{Add, AddAssign, Neg, Product, Sub, SubAssign, Sum};
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    PartialEq,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord2<T: Num>(pub T, pub T);

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    PartialEq,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord3<T: Num>(pub T, pub T, pub T);

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    PartialEq,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord4<T: Num>(pub T, pub T, pub T, pub T);

macro_rules! coord_impls {
    ($name:ident, $($field:tt),*) => {
        impl<T: Num> $name<T> {
            neighbors_fn!($($field)*);

            orthogonal_neighbors_fn!($($field)*);

            pub fn manhattan_norm(self) -> T {
                T::default() $(+ self.$field.abs())*
            }
        }

        impl<T: Num> Mul<T> for $name<T> {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                Self($(self.$field * rhs,)*)
            }
        }

        impl<T: Num> MulAssign<T> for $name<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$field *= rhs;)*
            }
        }

        impl<T: Num> Div<T> for $name<T> {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                Self($(self.$field / rhs,)*)
            }
        }

        impl<T: Num> DivAssign<T> for $name<T> {
            fn div_assign(&mut self, rhs: T) {
                $(self.$field /= rhs;)*
            }
        }
    };
}

macro_rules! neighbors_fn {
    (@inner $self:ident $result:ident $neighbor:ident) => {
        if $neighbor != $self {
            $result.push($neighbor);
        }
    };
    (@inner $self:ident $result:ident $neighbor:ident $head_field:tt $($rest_field:tt)*) => {
        for x in [$self.$head_field - T::from(1), $self.$head_field, $self.$head_field + T::from(1)] {
            $neighbor.$head_field = x;
            neighbors_fn!(@inner $self $result $neighbor $($rest_field)*);
        }
    };
    ($($field:tt)*) => {
        pub fn neighbors(self) -> Vec<Self> {
            let mut result = vec![];
            let mut neighbor = Self::default();
            neighbors_fn!(@inner self result neighbor $($field)*);
            result
        }
    };
}

macro_rules! orthogonal_neighbors_fn {
    ($($field:tt)*) => {
        pub fn orthogonal_neighbors(self) -> Vec<Self> {
            let mut result = vec![];
            $(
                for x in [self.$field - T::from(1), self.$field + T::from(1)] {
                    let mut neighbor = self;
                    neighbor.$field = x;
                    result.push(neighbor);
                }
            )*
            result
        }
    };
}

coord_impls!(Coord2, 0, 1);
coord_impls!(Coord3, 0, 1, 2);
coord_impls!(Coord4, 0, 1, 2, 3);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_coord2() {
        let c = Coord2(1, -2);
        assert_eq!(c + c, Coord2(2, -4));
        assert_eq!(c * 10, Coord2(10, -20));
        assert_eq!(c.manhattan_norm(), 3);
        let neighbors_set: HashSet<_> = c.neighbors().into_iter().collect();
        let neighbors_expected: HashSet<_> = [
            Coord2(0, -3),
            Coord2(0, -2),
            Coord2(0, -1),
            Coord2(1, -3),
            Coord2(1, -1),
            Coord2(2, -3),
            Coord2(2, -2),
            Coord2(2, -1),
        ]
        .into_iter()
        .collect();
        assert_eq!(neighbors_set, neighbors_expected);
        let neighbors_set: HashSet<_> = c.orthogonal_neighbors().into_iter().collect();
        let neighbors_expected: HashSet<_> =
            [Coord2(0, -3), Coord2(0, -1), Coord2(2, -3), Coord2(2, -1)]
                .into_iter()
                .collect();
        assert_eq!(neighbors_set, neighbors_expected);
    }

    #[test]
    fn test_coord3() {
        let c = Coord3(1, -2, 3);
        assert_eq!(c + c, Coord3(2, -4, 6));
        assert_eq!(c * 10, Coord3(10, -20, 30));
        assert_eq!(c.manhattan_norm(), 6)
    }
}
