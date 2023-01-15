use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Seconds {
    pub(crate) value: f64,
}

impl Seconds {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn new_millisecond(value: f64) -> Self {
        Self { value: value * 1e-3 }
    }

    pub fn new_microsecond(value: f64) -> Self {
        Self { value: value * 1e-6 }
    }

    pub fn new_nanosecond(value: f64) -> Self {
        Self { value: value * 1e-9 }
    }
}

impl Mul<Seconds> for Seconds {
    type Output = Seconds;

    fn mul(self, other: Seconds) -> Seconds {
        Seconds::new(self.value * other.value)
    }
}

impl MulAssign<Seconds> for Seconds {
    fn mul_assign(&mut self, other: Seconds) {
        *self = Seconds::new(self.value * other.value);
    }
}

impl Div<Seconds> for Seconds {
    type Output = Seconds;

    fn div(self, other: Seconds) -> Seconds {
        Seconds::new(self.value / other.value)
    }
}

impl DivAssign<Seconds> for Seconds {
    fn div_assign(&mut self, other: Seconds) {
        *self = Seconds::new(self.value / other.value);
    }
}

impl Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, other: Seconds) -> Seconds {
        Seconds::new(self.value + other.value)
    }
}

impl AddAssign<Seconds> for Seconds {
    fn add_assign(&mut self, other: Seconds) {
        *self = Seconds::new(self.value + other.value);
    }
}

impl Sub<Seconds> for Seconds {
    type Output = Seconds;

    fn sub(self, other: Seconds) -> Seconds {
        Seconds::new(self.value - other.value)
    }
}

impl SubAssign<Seconds> for Seconds {
    fn sub_assign(&mut self, other: Seconds) {
        *self = Seconds::new(self.value - other.value);
    }
}

impl Rem<Seconds> for Seconds {
    type Output = Seconds;

    fn rem(self, other: Seconds) -> Seconds {
        Seconds::new(self.value % other.value)
    }
}

impl RemAssign<Seconds> for Seconds {
    fn rem_assign(&mut self, other: Seconds) {
        *self = Seconds::new(self.value % other.value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn second_eq_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn second_ne_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_ne!(a, b);
    }

    #[test]
    fn second_mul_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_eq!(a * b, Seconds::new(2.0));
    }

    #[test]
    fn second_mul_assign_second() {
        let mut a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        a *= b;
        assert_eq!(a, Seconds::new(2.0));
    }

    #[test]
    fn second_div_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_eq!(a / b, Seconds::new(0.5));
    }

    #[test]
    fn second_div_assign_second() {
        let mut a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        a /= b;
        assert_eq!(a, Seconds::new(0.5));
    }

    #[test]
    fn second_add_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_eq!(a + b, Seconds::new(3.0));
    }

    #[test]
    fn second_add_assign_second() {
        let mut a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        a += b;
        assert_eq!(a, Seconds::new(3.0));
    }

    #[test]
    fn second_sub_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_eq!(a - b, Seconds::new(-1.0));
    }

    #[test]
    fn second_sub_assign_second() {
        let mut a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        a -= b;
        assert_eq!(a, Seconds::new(-1.0));
    }

    #[test]
    fn second_rem_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        assert_eq!(a % b, Seconds::new(1.0));
    }

    #[test]
    fn second_rem_assign_second() {
        let mut a = Seconds::new(1.0);
        let b = Seconds::new(2.0);
        a %= b;
        assert_eq!(a, Seconds::new(1.0));
    }
}