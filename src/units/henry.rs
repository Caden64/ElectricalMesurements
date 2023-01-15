use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
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
        *self = Henry::new(self.value * other.value);
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
        *self = Henry::new(self.value / other.value);
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
        *self = Henry::new(self.value + other.value);
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
        *self = Henry::new(self.value - other.value);
    }
}

impl Rem<Henry> for Henry {
    type Output = Henry;

    fn rem(self, other: Henry) -> Henry {
        Henry::new(self.value % other.value)
    }
}

impl RemAssign<Henry> for Henry {
    fn rem_assign(&mut self, other: Henry) {
        *self = Henry::new(self.value % other.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = a * b;
        assert_eq!(c.value, 2.0);
    }

    #[test]
    fn mul_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        a *= b;
        assert_eq!(a.value, 2.0);
    }

    #[test]
    fn div() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = a / b;
        assert_eq!(c.value, 0.5);
    }

    #[test]
    fn div_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        a /= b;
        assert_eq!(a.value, 0.5);
    }

    #[test]
    fn add() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = a + b;
        assert_eq!(c.value, 3.0);
    }

    #[test]
    fn add_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        a += b;
        assert_eq!(a.value, 3.0);
    }

    #[test]
    fn sub() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = a - b;
        assert_eq!(c.value, -1.0);
    }

    #[test]
    fn sub_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        a -= b;
        assert_eq!(a.value, -1.0);
    }

    #[test]
    fn rem() {
        let a = Henry::new(1.0);
        let b = Henry::new(2.0);
        let c = a % b;
        assert_eq!(c.value, 1.0);
    }

    #[test]
    fn rem_assign() {
        let mut a = Henry::new(1.0);
        let b = Henry::new(2.0);
        a %= b;
        assert_eq!(a.value, 1.0);
    }
}