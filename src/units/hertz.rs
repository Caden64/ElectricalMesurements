use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hertz {
    pub(crate) value: f64,
}

impl Hertz {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn new_kilohertz(value: f64) -> Self {
        Self { value: value * 1e3 }
    }

    pub fn new_megahertz(value: f64) -> Self {
        Self { value: value * 1e6 }
    }

    pub fn new_gigahertz(value: f64) -> Self {
        Self { value: value * 1e9 }
    }
}

impl Mul<Hertz> for Hertz {
    type Output = Hertz;

    fn mul(self, other: Hertz) -> Hertz {
        Hertz::new(self.value * other.value)
    }
}

impl MulAssign<Hertz> for Hertz {
    fn mul_assign(&mut self, other: Hertz) {
        *self = Hertz::new(self.value * other.value);
    }
}

impl Div<Hertz> for Hertz {
    type Output = Hertz;

    fn div(self, other: Hertz) -> Hertz {
        Hertz::new(self.value / other.value)
    }
}

impl DivAssign<Hertz> for Hertz {
    fn div_assign(&mut self, other: Hertz) {
        *self = Hertz::new(self.value / other.value);
    }
}

impl Add<Hertz> for Hertz {
    type Output = Hertz;

    fn add(self, other: Hertz) -> Hertz {
        Hertz::new(self.value + other.value)
    }
}

impl AddAssign<Hertz> for Hertz {
    fn add_assign(&mut self, other: Hertz) {
        *self = Hertz::new(self.value + other.value);
    }
}

impl Sub<Hertz> for Hertz {
    type Output = Hertz;

    fn sub(self, other: Hertz) -> Hertz {
        Hertz::new(self.value - other.value)
    }
}

impl SubAssign<Hertz> for Hertz {
    fn sub_assign(&mut self, other: Hertz) {
        *self = Hertz::new(self.value - other.value);
    }
}

impl Rem<Hertz> for Hertz {
    type Output = Hertz;

    fn rem(self, other: Hertz) -> Hertz {
        Hertz::new(self.value % other.value)
    }
}

impl RemAssign<Hertz> for Hertz {
    fn rem_assign(&mut self, other: Hertz) {
        *self = Hertz::new(self.value % other.value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hertz_eq_hertz() {
        let a = Hertz::new(1.0);
        let b = Hertz::new(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn one_thousand_hertz_eq_kilohertz() {
        let a = Hertz::new(1_000.0);
        let b = Hertz::new_kilohertz(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn one_million_hertz_eq_megahertz() {
        let a = Hertz::new(1_000_000.0);
        let b = Hertz::new_megahertz(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn one_billion_hertz_eq_gigahertz() {
        let a = Hertz::new(1_000_000_000.0);
        let b = Hertz::new_gigahertz(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn hertz_mul_hertz() {
        let a = Hertz::new(1.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(2.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn hertz_mul_assign_hertz() {
        let mut a = Hertz::new(1.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(2.0);
        a *= b;
        assert_eq!(a, c);
    }

    #[test]
    fn hertz_div_hertz() {
        let a = Hertz::new(2.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        assert_eq!(a / b, c);
    }

    #[test]
    fn hertz_div_assign_hertz() {
        let mut a = Hertz::new(2.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        a /= b;
        assert_eq!(a, c);
    }

    #[test]
    fn hertz_add_hertz() {
        let a = Hertz::new(1.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(3.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn hertz_add_assign_hertz() {
        let mut a = Hertz::new(1.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(3.0);
        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn hertz_sub_hertz() {
        let a = Hertz::new(3.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn hertz_sub_assign_hertz() {
        let mut a = Hertz::new(3.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        a -= b;
        assert_eq!(a, c);
    }

    #[test]
    fn hertz_rem_hertz() {
        let a = Hertz::new(3.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        assert_eq!(a % b, c);
    }

    #[test]
    fn hertz_rem_assign_hertz() {
        let mut a = Hertz::new(3.0);
        let b = Hertz::new(2.0);
        let c = Hertz::new(1.0);
        a %= b;
        assert_eq!(a, c);
    }

}
