use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Watt {
    pub value: f64,
}

impl Watt {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Watt> for Watt {
    type Output = Watt;

    fn mul(self, other: Watt) -> Watt {
        Watt::new(self.value * other.value)
    }
}

impl MulAssign<Watt> for Watt {
    fn mul_assign(&mut self, other: Watt) {
        self.value *= other.value;
    }
}

impl Div<Watt> for Watt {
    type Output = Watt;

    fn div(self, other: Watt) -> Watt {
        Watt::new(self.value / other.value)
    }
}

impl DivAssign<Watt> for Watt {
    fn div_assign(&mut self, other: Watt) {
        self.value /= other.value;
    }
}

impl Add<Watt> for Watt {
    type Output = Watt;

    fn add(self, other: Watt) -> Watt {
        Watt::new(self.value + other.value)
    }
}

impl AddAssign<Watt> for Watt {
    fn add_assign(&mut self, other: Watt) {
        self.value += other.value;
    }
}

impl Sub<Watt> for Watt {
    type Output = Watt;

    fn sub(self, other: Watt) -> Watt {
        Watt::new(self.value - other.value)
    }
}

impl SubAssign<Watt> for Watt {
    fn sub_assign(&mut self, other: Watt) {
        self.value -= other.value;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Watt::new(1.0);
        let b = Watt::new(2.0);
        let c = a * b;
        assert_eq!(c.value, 2.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Watt::new(1.0);
        let b = Watt::new(2.0);
        a *= b;
        assert_eq!(a.value, 2.0);
    }

    #[test]
    fn test_div() {
        let a = Watt::new(1.0);
        let b = Watt::new(2.0);
        let c = a / b;
        assert_eq!(c.value, 0.5);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Watt::new(1.0);
        let b = Watt::new(2.0);
        a /= b;
        assert_eq!(a.value, 0.5);
    }

    #[test]
    fn test_add() {
        let a = Watt::new(1.0);
        let b = Watt::new(2.0);
        let c = a + b;
        assert_eq!(c.value, 3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Watt::new(1.0);
        let b = Watt::new(2.0);
        a += b;
        assert_eq!(a.value, 3.0);
    }

    #[test]
    fn test_sub() {
        let a = Watt::new(1.0);
        let b = Watt::new(2.0);
        let c = a - b;
        assert_eq!(c.value, -1.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Watt::new(1.0);
        let b = Watt::new(2.0);
        a -= b;
        assert_eq!(a.value, -1.0);
    }

}