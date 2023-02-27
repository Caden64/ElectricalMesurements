use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

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
        self.value *= other.value;
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
        self.value /= other.value;
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
        self.value += other.value;
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
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Volt::new(1.0);
        let b = Volt::new(2.0);
        let c = a * b;
        assert_eq!(c, Volt::new(2.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Volt::new(1.0);
        let b = Volt::new(2.0);
        a *= b;
        assert_eq!(a, Volt::new(2.0));
    }

    #[test]
    fn test_div() {
        let a = Volt::new(1.0);
        let b = Volt::new(2.0);
        let c = a / b;
        assert_eq!(c, Volt::new(0.5));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Volt::new(1.0);
        let b = Volt::new(2.0);
        a /= b;
        assert_eq!(a, Volt::new(0.5));
    }

    #[test]
    fn test_add() {
        let a = Volt::new(1.0);
        let b = Volt::new(2.0);
        let c = a + b;
        assert_eq!(c, Volt::new(3.0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Volt::new(1.0);
        let b = Volt::new(2.0);
        a += b;
        assert_eq!(a, Volt::new(3.0));
    }

    #[test]
    fn test_sub() {
        let a = Volt::new(1.0);
        let b = Volt::new(2.0);
        let c = a - b;
        assert_eq!(c, Volt::new(-1.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Volt::new(1.0);
        let b = Volt::new(2.0);
        a -= b;
        assert_eq!(a, Volt::new(-1.0));
    }
}