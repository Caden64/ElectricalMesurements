use std::fmt;
use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Volt {
    pub value: f64,
}

impl Volt {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Volt> for Volt {
    type Output = Volt;

    fn mul(self, other: Volt) -> Volt {
        Volt::new(self.value * other.value)
    }
}

impl MulAssign<Volt> for Volt {
    fn mul_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value * other.value);
    }
}

impl Div<Volt> for Volt {
    type Output = Volt;

    fn div(self, other: Volt) -> Volt {
        Volt::new(self.value / other.value)
    }
}

impl DivAssign<Volt> for Volt {
    fn div_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value / other.value);
    }
}

impl Add<Volt> for Volt {
    type Output = Volt;

    fn add(self, other: Volt) -> Volt {
        Volt::new(self.value + other.value)
    }
}

impl AddAssign<Volt> for Volt {
    fn add_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value + other.value);
    }
}

impl Sub<Volt> for Volt {
    type Output = Volt;

    fn sub(self, other: Volt) -> Volt {
        Volt::new(self.value - other.value)
    }
}

impl SubAssign<Volt> for Volt {
    fn sub_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value - other.value);
    }
}

impl Rem<Volt> for Volt {
    type Output = Volt;

    fn rem(self, other: Volt) -> Volt {
        Volt::new(self.value % other.value)
    }
}

impl RemAssign<Volt> for Volt {
    fn rem_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value % other.value);
    }
}

impl fmt::Display for Volt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.} V", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volts_eq_volts() {
        assert_eq!(Volt::new(5.0).value, 5.0)
    }

    #[test]
    fn volts_add() {
        assert_eq!(Volt::new(5.0) + Volt::new(3.0), Volt::new(8.0))
    }

    #[test]
    fn volts_add_neg() {
        assert_eq!(Volt::new(-5.0) + Volt::new(3.0), Volt::new(-2.0))
    }

    #[test]
    fn volts_add_assign() {
        let mut test_value = Volt::new(5.0);
        test_value += Volt::new(3.0);
        assert_eq!(test_value, Volt::new(8.0))
    }

    #[test]
    fn volts_add_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value += Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-2.0))
    }

    #[test]
    fn volts_sub() {
        assert_eq!(Volt::new(5.0) - Volt::new(3.0), Volt::new(2.0))
    }

    #[test]
    fn volts_sub_neg() {
        assert_eq!(Volt::new(-5.0) - Volt::new(3.0), Volt::new(-8.0))
    }

    #[test]
    fn volts_sub_assign() {
        let mut test_value = Volt::new(5.0);
        test_value -= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(2.0))
    }

    #[test]
    fn volts_sub_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value -= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-8.0))
    }

    #[test]
    fn volts_mul() {
        assert_eq!(Volt::new(5.0) * Volt::new(3.0), Volt::new(15.0))
    }

    #[test]
    fn volts_mul_neg() {
        assert_eq!(Volt::new(-5.0) * Volt::new(3.0), Volt::new(-15.0))
    }

    #[test]
    fn volts_mul_assign() {
        let mut test_value = Volt::new(5.0);
        test_value *= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(15.0))
    }

    #[test]
    fn volts_mul_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value *= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-15.0))
    }

    #[test]
    fn volts_div() {
        assert_eq!(Volt::new(5.0) / Volt::new(3.0), Volt::new(5.0 / 3.0))
    }

    #[test]
    fn volts_div_neg() {
        assert_eq!(Volt::new(-5.0) / Volt::new(3.0), Volt::new(-5.0 / 3.0))
    }

    #[test]
    fn volts_div_assign() {
        let mut test_value = Volt::new(5.0);
        test_value /= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(5.0 / 3.0))
    }

    #[test]
    fn volts_div_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value /= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-5.0 / 3.0))
    }

    #[test]
    fn volts_rem() {
        assert_eq!(Volt::new(5.0) % Volt::new(3.0), Volt::new(2.0))
    }

    #[test]
    fn volts_rem_neg() {
        assert_eq!(Volt::new(-5.0) % Volt::new(3.0), Volt::new(-2.0))
    }

    #[test]
    fn volts_rem_assign() {
        let mut test_value = Volt::new(5.0);
        test_value %= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(2.0))
    }

    #[test]
    fn volts_rem_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value %= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-2.0))
    }

    #[test]
    fn volts_display() {
        assert_eq!(format!("{}", Volt::new(5.0)), "5 V")
    }

}