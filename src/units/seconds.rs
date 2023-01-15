use std::ops::{Mul, MulAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Seconds {
    value: f64,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn second_eq_second() {
        let a = Seconds::new(1.0);
        let b = Seconds::new(1.0);
        assert_eq!(a, b);
    }
}