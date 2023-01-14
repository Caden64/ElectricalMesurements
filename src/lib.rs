mod macros;

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

measure!(Ohm, "Ω");

#[cfg(test)]
mod tests {
    use crate::*;

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