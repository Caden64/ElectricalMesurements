use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
macro_rules! measure {
    ($id:ident, $symbol:literal) => {
        #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
        pub struct $id(pub f64);

        impl fmt::Display for $id {
            fn fmt(&self, sdt: &mut fmt::Formatter) -> fmt::Result {
                write!(sdt, "{:.2} {}", self.0, $symbol)
            }
        }

        impl Add for $id {
           type Output = $id;

           fn add(self, other: $id) -> $id {
               $id (self.0 + other.0)
           }
        }

        impl AddAssign for $id {
            fn add_assign(&mut self, other: $id) {
                *self = $id (self.0 + other.0);
            }
        }

        impl Sub for $id {
            type Output = $id;

            fn sub(self, other: $id) -> $id {
                $id(self.0 - other.0)
            }
        }

        impl SubAssign for $id {
            fn sub_assign(&mut self, other: $id) {
                *self = $id(self.0 - other.0);
            }
        }

        impl Mul for $id {
            type Output = $id;

            fn mul(self, other: $id) -> $id {
                $id(self.0 * other.0)
            }
        }

        impl MulAssign for $id {
            fn mul_assign(&mut self, other: $id) {
                *self = $id(self.0 * other.0);
            }
        }

        impl Div for $id {
            type Output = $id;

            fn div(self, other: $id) -> $id {
                $id(self.0 / other.0)
            }
        }

        impl DivAssign for $id {
            fn div_assign(&mut self, other: $id) {
                *self = $id(self.0 / other.0);
            }
        }
    }
}

measure!(Ohm, "Ω");

#[cfg(test)]
mod tests {
    use crate::Ohm;

    #[test]
    fn ohm_eq_ohm() {
        assert_eq!(Ohm(5.0).0, 5.0)
    }

    #[test]
    fn ohm_add() {
        assert_eq!(Ohm(5.0) + Ohm(3.0), Ohm(8.0))
    }

    #[test]
    fn ohm_add_neg() {
        assert_eq!(Ohm(-5.0) + Ohm(3.0), Ohm(-2.0))
    }
    #[test]

    fn ohm_add_assign() {
        let mut test_value = Ohm(5.0);
        test_value += Ohm(3.0);
        assert_eq!(test_value, Ohm(8.0))
    }

    #[test]
    fn ohm_add_assign_neg() {
        let mut test_value = Ohm(-5.0);
        test_value += Ohm(3.0);
        assert_eq!(test_value, Ohm(-2.0))
    }

    #[test]
    fn ohm_sub() {
        assert_eq!(Ohm(5.0) - Ohm(3.0), Ohm(2.0))
    }

    #[test]
    fn ohm_sub_neg() {
        assert_eq!(Ohm(-5.0) - Ohm(3.0), Ohm(-8.0))
    }

    #[test]
    fn  ohm_sub_assign() {
        let mut test_value = Ohm(5.0);
        test_value -= Ohm(3.0);
        assert_eq!(test_value, Ohm(2.0))
    }

    #[test]
    fn ohm_sub_assign_neg() {
        let mut test_value = Ohm(-5.0);
        test_value -= Ohm(3.0);
        assert_eq!(test_value, Ohm(-8.0))
    }

    #[test]
    fn ohm_mul() {
        assert_eq!(Ohm(5.0) * Ohm(3.0), Ohm(15.0))
    }

    #[test]
    fn ohm_mul_neg() {
        assert_eq!(Ohm(-5.0) * Ohm(3.0), Ohm(-15.0))
    }

    #[test]
    fn ohm_mul_assign() {
        let mut test_value = Ohm(5.0);
        test_value *= Ohm(3.0);
        assert_eq!(test_value, Ohm(15.0))
    }

    #[test]
    fn ohm_mul_assign_neg() {
        let mut test_value = Ohm(-5.0);
        test_value *= Ohm(3.0);
        assert_eq!(test_value, Ohm(-15.0))
    }

    #[test]
    fn ohm_div() {
        assert_eq!(Ohm(5.0) / Ohm(3.0), Ohm(1.6666666666666667))
    }

    #[test]
    fn ohm_div_neg() {
        assert_eq!(Ohm(-5.0) / Ohm(3.0), Ohm(-1.6666666666666667))
    }

    #[test]
    fn ohm_div_assign() {
        let mut test_value = Ohm(5.0);
        test_value /= Ohm(3.0);
        assert_eq!(test_value, Ohm(1.6666666666666667))
    }

    #[test]
    fn ohm_div_assign_neg() {
        let mut test_value = Ohm(-5.0);
        test_value /= Ohm(3.0);
        assert_eq!(test_value, Ohm(-1.6666666666666667))
    }
}