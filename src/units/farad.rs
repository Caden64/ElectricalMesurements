use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Farad {
    pub value: f64,
}

impl Farad {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Farad> for Farad {
    type Output = Farad;

    fn mul(self, other: Farad) -> Farad {
        Farad::new(self.value * other.value)
    }
}

impl MulAssign<Farad> for Farad {
    fn mul_assign(&mut self, other: Farad) {
        self.value *= other.value;
    }
}

impl Div<Farad> for Farad {
    type Output = Farad;

    fn div(self, other: Farad) -> Farad {
        Farad::new(self.value / other.value)
    }
}

impl DivAssign<Farad> for Farad {
    fn div_assign(&mut self, other: Farad) {
        self.value /= other.value;
    }
}

impl Add<Farad> for Farad {
    type Output = Farad;

    fn add(self, other: Farad) -> Farad {
        Farad::new(self.value + other.value)
    }
}

impl AddAssign<Farad> for Farad {
    fn add_assign(&mut self, other: Farad) {
        self.value += other.value;
    }
}

impl Sub<Farad> for Farad {
    type Output = Farad;

    fn sub(self, other: Farad) -> Farad {
        Farad::new(self.value - other.value)
    }
}

impl SubAssign<Farad> for Farad {
    fn sub_assign(&mut self, other: Farad) {
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        let c = Farad::new(2.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        let c = Farad::new(2.0);
        a *= b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_div() {
        let a = Farad::new(3.0);
        let b = Farad::new(2.0);
        let c = Farad::new(1.5);
        assert_eq!(a / b, c);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Farad::new(3.0);
        let b = Farad::new(2.0);
        let c = Farad::new(1.5);
        a /= b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_add() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        let c = Farad::new(3.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        let c = Farad::new(3.0);
        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_sub() {
        let a = Farad::new(3.0);
        let b = Farad::new(2.0);
        let c = Farad::new(1.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Farad::new(3.0);
        let b = Farad::new(2.0);
        a -= b;
        let c = Farad::new(1.0);
        assert_eq!(a, c)
    }
}