use std::ops::{Mul, MulAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Farad {
    value: f64,
}

impl Farad {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn new_microfarad(value: f64) -> Self {
        Self { value: value * 1e-6 }
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
        *self = Farad::new(self.value * other.value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn farad_eq_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(1.0);
        assert_eq!(a, b);
    }
}