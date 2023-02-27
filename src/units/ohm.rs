use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Ohm {
    pub value: f64,
}

impl Ohm {
    pub fn new(value: f64) -> Self {
        Self { value }
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
        self.value *= other.value;
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
        self.value /= other.value;
    }
}

impl Add<Ohm> for Ohm {
    type Output = Ohm;

    fn add(self, other: Ohm) -> Ohm {
        Ohm::new(self.value + other.value)
    }
}

impl AddAssign<Ohm> for Ohm {
    fn add_assign(&mut self, other: Ohm) {
        self.value += other.value;
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
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        let c = a * b;
        assert_eq!(c.value, 6.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        a *= b;
        assert_eq!(a.value, 6.0);
    }

    #[test]
    fn test_div() {
        let a = Ohm::new(6.0);
        let b = Ohm::new(3.0);
        let c = a / b;
        assert_eq!(c.value, 2.0);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Ohm::new(6.0);
        let b = Ohm::new(3.0);
        a /= b;
        assert_eq!(a.value, 2.0);
    }

    #[test]
    fn test_add() {
        let a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        let c = a + b;
        assert_eq!(c.value, 5.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        a += b;
        assert_eq!(a.value, 5.0);
    }

    #[test]
    fn test_sub() {
        let a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        let c = a - b;
        assert_eq!(c.value, -1.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Ohm::new(2.0);
        let b = Ohm::new(3.0);
        a -= b;
        assert_eq!(a.value, -1.0);
    }
}