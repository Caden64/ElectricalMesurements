use std::fmt;
use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Ohm {
    pub value: f64,
}

impl Ohm {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn reciprical(&self) -> Self {
        Self { value: 1.0 / self.value }
    }
}

impl Mul<Ohm> for Ohm {
    type Output = Ohm;

    fn mul(self, other: Ohm) -> Ohm {
        Ohm::new(self.value * other.value)
    }
}

impl MulAssign<Ohm> for Ohm {
    fn mul_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value * other.value);
    }
}

impl Div<Ohm> for Ohm {
    type Output = Ohm;

    fn div(self, other: Ohm) -> Ohm {
        Ohm::new(self.value / other.value)
    }
}

impl DivAssign<Ohm> for Ohm {
    fn div_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value / other.value);
    }
}

impl Add<Ohm> for Ohm {
    type Output = Ohm;

    fn add(self, other: Ohm) -> Ohm {
        Ohm::new(self.value + other.value)
    }
}
impl Div<Ohm> for f64 {
    type Output = Ohm;

    fn div(self, other: Ohm) -> Self::Output {
        Ohm::new(self / other.value)
    }
}
impl AddAssign<Ohm> for Ohm {
    fn add_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value + other.value);
    }
}

impl Sub<Ohm> for Ohm {
    type Output = Ohm;

    fn sub(self, other: Ohm) -> Ohm {
        Ohm::new(self.value - other.value)
    }
}

impl SubAssign<Ohm> for Ohm {
    fn sub_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value - other.value);
    }
}

impl Rem<Ohm> for Ohm {
    type Output = Ohm;

    fn rem(self, other: Ohm) -> Ohm {
        Ohm::new(self.value % other.value)
    }
}

impl RemAssign<Ohm> for Ohm {
    fn rem_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value % other.value);
    }
}

impl fmt::Display for Ohm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.} Ω", self.value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohms_eq_ohms() {
        assert_eq!(Ohm::new(5.0).value, 5.0)
    }

    #[test]
    fn ohms_add() {
        assert_eq!(Ohm::new(5.0) + Ohm::new(3.0), Ohm::new(8.0))
    }

    #[test]
    fn ohms_add_neg() {
        assert_eq!(Ohm::new(-5.0) + Ohm::new(3.0), Ohm::new(-2.0))
    }

    #[test]
    fn ohms_add_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value += Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(8.0))
    }

    #[test]
    fn ohms_add_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value += Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-2.0))
    }

    #[test]
    fn ohms_sub() {
        assert_eq!(Ohm::new(5.0) - Ohm::new(3.0), Ohm::new(2.0))
    }

    #[test]
    fn ohms_sub_neg() {
        assert_eq!(Ohm::new(-5.0) - Ohm::new(3.0), Ohm::new(-8.0))
    }

    #[test]
    fn ohms_sub_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value -= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(2.0))
    }

    #[test]
    fn ohms_sub_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value -= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-8.0))
    }

    #[test]
    fn ohms_mul() {
        assert_eq!(Ohm::new(5.0) * Ohm::new(3.0), Ohm::new(15.0))
    }

    #[test]
    fn ohms_mul_neg() {
        assert_eq!(Ohm::new(-5.0) * Ohm::new(3.0), Ohm::new(-15.0))
    }

    #[test]
    fn ohms_mul_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value *= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(15.0))
    }

    #[test]
    fn ohms_mul_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value *= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-15.0))
    }

    #[test]
    fn ohms_div() {
        assert_eq!(Ohm::new(5.0) / Ohm::new(3.0), Ohm::new(5.0 / 3.0))
    }

    #[test]
    fn ohms_div_neg() {
        assert_eq!(Ohm::new(-5.0) / Ohm::new(3.0), Ohm::new(-5.0 / 3.0))
    }

    #[test]
    fn ohms_div_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value /= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(5.0 / 3.0))
    }

    #[test]
    fn ohms_div_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value /= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-5.0 / 3.0))
    }

    #[test]
    fn ohms_rem() {
        assert_eq!(Ohm::new(5.0) % Ohm::new(3.0), Ohm::new(2.0))
    }

    #[test]
    fn ohms_rem_neg() {
        assert_eq!(Ohm::new(-5.0) % Ohm::new(3.0), Ohm::new(-2.0))
    }

    #[test]
    fn ohms_rem_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value %= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(2.0))
    }

    #[test]
    fn ohms_rem_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value %= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-2.0))
    }

    #[test]
    fn ohms_display() {
        assert_eq!(format!("{}", Ohm::new(5.0)), "5 Ω")
    }
}