use std::ops::{Mul, MulAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hertz {
    value: f64,
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
}
