use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug, Copy, Clone)]

pub struct Amp {
    pub value: f64,
}

impl Amp {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Amp> for Amp {
    type Output = Amp;

    fn mul(self, other: Amp) -> Amp {
        Amp::new(self.value * other.value)
    }
}

impl MulAssign<Amp> for Amp {
    fn mul_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value * other.value);
    }
}

impl Div<Amp> for Amp {
    type Output = Amp;

    fn div(self, other: Amp) -> Amp {
        Amp::new(self.value / other.value)
    }
}

impl DivAssign<Amp> for Amp {
    fn div_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value / other.value);
    }
}

impl Add<Amp> for Amp {
    type Output = Amp;

    fn add(self, other: Amp) -> Amp {
        Amp::new(self.value + other.value)
    }
}

impl AddAssign<Amp> for Amp {
    fn add_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value + other.value);
    }
}

impl Sub<Amp> for Amp {
    type Output = Amp;

    fn sub(self, other: Amp) -> Amp {
        Amp::new(self.value - other.value)
    }
}

impl SubAssign<Amp> for Amp {
    fn sub_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value - other.value);
    }
}

impl Rem<Amp> for Amp {
    type Output = Amp;

    fn rem(self, other: Amp) -> Amp {
        Amp::new(self.value % other.value)
    }
}

impl RemAssign<Amp> for Amp {
    fn rem_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value % other.value);
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn amps_eq_amps() {
        assert_eq!(Amp::new(5.0).value, 5.0)
    }

    #[test]
    fn amps_add() {
        assert_eq!(Amp::new(5.0) + Amp::new(3.0), Amp::new(8.0))
    }

    #[test]
    fn amps_add_neg() {
        assert_eq!(Amp::new(-5.0) + Amp::new(3.0), Amp::new(-2.0))
    }

    #[test]
    fn amps_add_assign() {
        let mut test_value = Amp::new(5.0);
        test_value += Amp::new(3.0);
        assert_eq!(test_value, Amp::new(8.0))
    }

    #[test]
    fn amps_add_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value += Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-2.0))
    }

    #[test]
    fn amps_sub() {
        assert_eq!(Amp::new(5.0) - Amp::new(3.0), Amp::new(2.0))
    }

    #[test]
    fn amps_sub_neg() {
        assert_eq!(Amp::new(-5.0) - Amp::new(3.0), Amp::new(-8.0))
    }

    #[test]
    fn amps_sub_assign() {
        let mut test_value = Amp::new(5.0);
        test_value -= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(2.0))
    }

    #[test]
    fn amps_sub_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value -= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-8.0))
    }

    #[test]
    fn amps_mul() {
        assert_eq!(Amp::new(5.0) * Amp::new(3.0), Amp::new(15.0))
    }

    #[test]
    fn amps_mul_neg() {
        assert_eq!(Amp::new(-5.0) * Amp::new(3.0), Amp::new(-15.0))
    }

    #[test]
    fn amps_mul_assign() {
        let mut test_value = Amp::new(5.0);
        test_value *= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(15.0))
    }

    #[test]
    fn amps_mul_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value *= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-15.0))
    }

    #[test]
    fn amps_div() {
        assert_eq!(Amp::new(5.0) / Amp::new(3.0), Amp::new(5.0 / 3.0))
    }

    #[test]
    fn amps_div_neg() {
        assert_eq!(Amp::new(-5.0) / Amp::new(3.0), Amp::new(-5.0 / 3.0))
    }

    #[test]
    fn amps_div_assign() {
        let mut test_value = Amp::new(5.0);
        test_value /= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(5.0 / 3.0))
    }

    #[test]
    fn amps_div_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value /= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-5.0 / 3.0))
    }

    #[test]
    fn amps_rem() {
        assert_eq!(Amp::new(5.0) % Amp::new(3.0), Amp::new(2.0))
    }

    #[test]
    fn amps_rem_neg() {
        assert_eq!(Amp::new(-5.0) % Amp::new(3.0), Amp::new(-2.0))
    }

    #[test]
    fn amps_rem_assign() {
        let mut test_value = Amp::new(5.0);
        test_value %= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(2.0))
    }

    #[test]
    fn amps_rem_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value %= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-2.0))
    }
}