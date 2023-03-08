use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
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
        self.value *= other.value;
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
        self.value /= other.value;
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
        self.value += other.value;
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
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Amp::new(1.0);
        let b = Amp::new(2.0);
        let c = a * b;
        assert_eq!(c.value, 2.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Amp::new(1.0);
        let b = Amp::new(2.0);
        a *= b;
        assert_eq!(a.value, 2.0);
    }

    #[test]
    fn test_div() {
        let a = Amp::new(1.0);
        let b = Amp::new(2.0);
        let c = a / b;
        assert_eq!(c.value, 0.5);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Amp::new(1.0);
        let b = Amp::new(2.0);
        a /= b;
        assert_eq!(a.value, 0.5);
    }

    #[test]
    fn test_add() {
        let a = Amp::new(1.0);
        let b = Amp::new(2.0);
        let c = a + b;
        assert_eq!(c.value, 3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Amp::new(1.0);
        let b = Amp::new(2.0);
        a += b;
        assert_eq!(a.value, 3.0);
    }

    #[test]
    fn test_sub() {
        let a = Amp::new(1.0);
        let b = Amp::new(2.0);
        let c = a - b;
        assert_eq!(c.value, -1.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Amp::new(1.0);
        let b = Amp::new(2.0);
        a -= b;
        assert_eq!(a.value, -1.0);
    }
}