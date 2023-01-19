use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Farad {
    pub(crate) value: f64,
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

impl Div<Farad> for Farad {
    type Output = Farad;

    fn div(self, other: Farad) -> Farad {
        Farad::new(self.value / other.value)
    }
}

impl DivAssign<Farad> for Farad {
    fn div_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value / other.value);
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
        *self = Farad::new(self.value + other.value);
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
        *self = Farad::new(self.value - other.value);
    }
}

impl Rem<Farad> for Farad {
    type Output = Farad;

    fn rem(self, other: Farad) -> Farad {
        Farad::new(self.value % other.value)
    }
}

impl RemAssign<Farad> for Farad {
    fn rem_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value % other.value);
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

    #[test]
    fn micro_farad_eq_farad() {
        let a = Farad::new_microfarad(1.0);
        let b = Farad::new(1e-6);
        assert_eq!(a, b);
    }

    #[test]
    fn farad_times_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a * b, Farad::new(2.0));
    }

    #[test]
    fn farad_assign_times_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a *= b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_divide_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a / b, Farad::new(0.5));
    }

    #[test]
    fn farad_assign_divide_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a /= b;
        assert_eq!(a, Farad::new(0.5));
    }

    #[test]
    fn farad_add_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a + b, Farad::new(3.0));
    }

    #[test]
    fn farad_assign_add_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a += b;
        assert_eq!(a, Farad::new(3.0));
    }

    #[test]
    fn farad_sub_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a - b, Farad::new(-1.0));
    }

    #[test]
    fn farad_assign_sub_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a -= b;
        assert_eq!(a, Farad::new(-1.0));
    }

    #[test]
    fn farad_rem_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a % b, Farad::new(1.0));
    }

    #[test]
    fn farad_assign_rem_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a %= b;
        assert_eq!(a, Farad::new(1.0));
    }

}