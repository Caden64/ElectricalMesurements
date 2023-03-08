use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Henry {
    pub value: f64,
}

impl Henry {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Henry> for Henry {
    type Output = Henry;

    fn mul(self, other: Henry) -> Henry {
        Henry::new(self.value * other.value)
    }
}

impl MulAssign<Henry> for Henry {
    fn mul_assign(&mut self, other: Henry) {
        self.value *= other.value;
    }
}

impl Div<Henry> for Henry {
    type Output = Henry;

    fn div(self, other: Henry) -> Henry {
        Henry::new(self.value / other.value)
    }
}

impl DivAssign<Henry> for Henry {
    fn div_assign(&mut self, other: Henry) {
        self.value /= other.value;
    }
}

impl Add<Henry> for Henry {
    type Output = Henry;

    fn add(self, other: Henry) -> Henry {
        Henry::new(self.value + other.value)
    }
}

impl AddAssign<Henry> for Henry {
    fn add_assign(&mut self, other: Henry) {
        self.value += other.value;
    }
}

impl Sub<Henry> for Henry {
    type Output = Henry;

    fn sub(self, other: Henry) -> Henry {
        Henry::new(self.value - other.value)
    }
}

impl SubAssign<Henry> for Henry {
    fn sub_assign(&mut self, other: Henry) {
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(2.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(2.0);
        a *= b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_div() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(0.5);
        assert_eq!(a / b, c);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(0.5);
        a /= b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_add() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(3.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = Henry::new(3.0);
        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn test_sub() {
        let a = Henry::new(2.0);
        let b = Henry::new(1.0);
        let c = Henry::new(1.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Henry::new(2.0);
        let b = Henry::new(1.0);
        let c = Henry::new(1.0);
        a -= b;
        assert_eq!(c, a);
    }
}