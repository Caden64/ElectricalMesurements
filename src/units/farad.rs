use std::fmt;
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

    pub fn reciprocal(&self) -> Self {
        Self { value: 1.0 / self.value }
    }
}

impl Mul<Farad> for Farad {
    type Output = Farad;

    fn mul(self, other: Farad) -> Farad {
        Farad::new(self.value * other.value)
    }
}

#[cfg(feature = "default_math")]
impl Mul<Farad> for f64 {
    type Output = Farad;

    fn mul(self, other: Farad) -> Farad {
        Farad::new(self * other.value)
    }
}

#[cfg(feature = "default_math")]
impl Mul<f64> for Farad {
    type Output = Farad;

    fn mul(self, other: f64) -> Farad {
        Farad::new(self.value * other)
    }
}

impl MulAssign<Farad> for Farad {
    fn mul_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value * other.value);
    }
}

#[cfg(feature = "default_math")]
impl MulAssign<Farad> for f64 {
    fn mul_assign(&mut self, other: Farad) {
        *self = *self * other.value;
    }
}

#[cfg(feature = "default_math")]
impl MulAssign<f64> for Farad {
    fn mul_assign(&mut self, other: f64) {
        *self = Farad::new(self.value * other);
    }
}

impl Div<Farad> for Farad {
    type Output = Farad;

    fn div(self, other: Farad) -> Farad {
        Farad::new(self.value / other.value)
    }
}

#[cfg(feature = "default_math")]
impl Div<Farad> for f64 {
    type Output = Farad;

    fn div(self, other: Farad) -> Farad {
        Farad::new(self / other.value)
    }
}

#[cfg(feature = "default_math")]
impl Div<f64> for Farad {
    type Output = Farad;

    fn div(self, other: f64) -> Farad {
        Farad::new(self.value / other)
    }
}

impl DivAssign<Farad> for Farad {
    fn div_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value / other.value);
    }
}

#[cfg(feature = "default_math")]
impl DivAssign<Farad> for f64 {
    fn div_assign(&mut self, other: Farad) {
        *self = *self / other.value;
    }
}

#[cfg(feature = "default_math")]
impl DivAssign<f64> for Farad {
    fn div_assign(&mut self, other: f64) {
        *self = Farad::new(self.value / other);
    }
}

impl Add<Farad> for Farad {
    type Output = Farad;

    fn add(self, other: Farad) -> Farad {
        Farad::new(self.value + other.value)
    }
}

#[cfg(feature = "default_math")]
impl Add<Farad> for f64 {
    type Output = Farad;

    fn add(self, other: Farad) -> Farad {
        Farad::new(self + other.value)
    }
}

#[cfg(feature = "default_math")]
impl Add<f64> for Farad {
    type Output = Farad;

    fn add(self, other: f64) -> Farad {
        Farad::new(self.value + other)
    }
}

impl AddAssign<Farad> for Farad {
    fn add_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value + other.value);
    }
}

#[cfg(feature = "default_math")]
impl AddAssign<Farad> for f64 {
    fn add_assign(&mut self, other: Farad) {
        *self = *self + other.value;
    }
}

#[cfg(feature = "default_math")]
impl AddAssign<f64> for Farad {
    fn add_assign(&mut self, other: f64) {
        *self = Farad::new(self.value + other);
    }
}

impl Sub<Farad> for Farad {
    type Output = Farad;

    fn sub(self, other: Farad) -> Farad {
        Farad::new(self.value - other.value)
    }
}

#[cfg(feature = "default_math")]
impl Sub<Farad> for f64 {
    type Output = Farad;

    fn sub(self, other: Farad) -> Farad {
        Farad::new(self - other.value)
    }
}

#[cfg(feature = "default_math")]
impl Sub<f64> for Farad {
    type Output = Farad;

    fn sub(self, other: f64) -> Farad {
        Farad::new(self.value - other)
    }
}

impl SubAssign<Farad> for Farad {
    fn sub_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value - other.value);
    }
}

#[cfg(feature = "default_math")]
impl SubAssign<Farad> for f64 {
    fn sub_assign(&mut self, other: Farad) {
        *self = *self - other.value;
    }
}

#[cfg(feature = "default_math")]
impl SubAssign<f64> for Farad {
    fn sub_assign(&mut self, other: f64) {
        *self = Farad::new(self.value - other);
    }
}

impl Rem<Farad> for Farad {
    type Output = Farad;

    fn rem(self, other: Farad) -> Farad {
        Farad::new(self.value % other.value)
    }
}

#[cfg(feature = "default_math")]
impl Rem<Farad> for f64 {
    type Output = Farad;

    fn rem(self, other: Farad) -> Farad {
        Farad::new(self % other.value)
    }
}

#[cfg(feature = "default_math")]
impl Rem<f64> for Farad {
    type Output = Farad;

    fn rem(self, other: f64) -> Farad {
        Farad::new(self.value % other)
    }
}

impl RemAssign<Farad> for Farad {
    fn rem_assign(&mut self, other: Farad) {
        *self = Farad::new(self.value % other.value);
    }
}

#[cfg(feature = "default_math")]
impl RemAssign<Farad> for f64 {
    fn rem_assign(&mut self, other: Farad) {
        *self = *self % other.value;
    }
}

#[cfg(feature = "default_math")]
impl RemAssign<f64> for Farad {
    fn rem_assign(&mut self, other: f64) {
        *self = Farad::new(self.value % other);
    }
}

impl fmt::Display for Farad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value > 0.0001 {
            write!(f, "{} F", self.value)
        } else {
            write!(f, "{} µF", self.value * 1e6)
        }

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
    fn farad_add_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(1.0);
        assert_eq!(a + b, Farad::new(2.0));
    }

    #[test]
    fn farad_add_neg() {
        let a = Farad::new(1.0);
        let b = Farad::new(-1.0);
        assert_eq!(a + b, Farad::new(0.0));
    }

    #[test]
    fn farad_add_micro_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new_microfarad(1.0);
        assert_eq!(a + b, Farad::new(1.000001));
    }

    #[test]
    fn farad_add_assign_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(1.0);
        a += b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_add_assign_neg() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(-1.0);
        a += b;
        assert_eq!(a, Farad::new(0.0));
    }

    #[test]
    fn farad_add_assign_micro_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new_microfarad(1.0);
        a += b;
        assert_eq!(a, Farad::new(1.000001));
    }

    #[test]
    fn farad_add_assign_f64() {
        let mut a = Farad::new(1.0);
        let b = 1.0;
        a += b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_add_assign_f64_neg() {
        let mut a = Farad::new(1.0);
        let b = -1.0;
        a += b;
        assert_eq!(a, Farad::new(0.0));
    }

    #[test]
    fn farad_add_assign_f64_micro_farad() {
        let mut a = Farad::new(1.0);
        let b = 1e-6;
        a += b;
        assert_eq!(a, Farad::new(1.000001));
    }

    #[test]
    fn farad_sub_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(1.0);
        assert_eq!(a - b, Farad::new(0.0));
    }

    #[test]
    fn farad_sub_neg() {
        let a = Farad::new(1.0);
        let b = Farad::new(-1.0);
        assert_eq!(a - b, Farad::new(2.0));
    }

    #[test]
    fn farad_sub_micro_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new_microfarad(1.0);
        assert_eq!(a - b, Farad::new(0.999999));
    }

    #[test]
    fn farad_sub_f64_neg() {
        let a = Farad::new(1.0);
        let b = -1.0;
        assert_eq!(a - b, Farad::new(2.0));
    }

    #[test]
    fn farad_sub_f64_micro_farad() {
        let a = Farad::new(1.0);
        let b = 1e-6;
        assert_eq!(a - b, Farad::new(0.999999));
    }

    #[test]
    fn farad_sub_assign_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(1.0);
        a -= b;
        assert_eq!(a, Farad::new(0.0));
    }

    #[test]
    fn farad_sub_assign_neg() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(-1.0);
        a -= b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_sub_assign_micro_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new_microfarad(1.0);
        a -= b;
        assert_eq!(a, Farad::new(0.999999));
    }

    #[test]
    fn farad_sub_assign_f64_neg() {
        let mut a = Farad::new(1.0);
        let b = -1.0;
        a -= b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_sub_assign_f64_micro_farad() {
        let mut a = Farad::new(1.0);
        let b = 1e-6;
        a -= b;
        assert_eq!(a, Farad::new(0.999999));
    }

    #[test]
    fn farad_mul_f64() {
        let a = Farad::new(1.0);
        let b = 2.0;
        assert_eq!(a * b, Farad::new(2.0));
    }

    #[test]
    fn farad_mul_assign_f64() {
        let mut a = Farad::new(1.0);
        let b = 2.0;
        a *= b;
        assert_eq!(a, Farad::new(2.0));
    }

    #[test]
    fn farad_div_f64() {
        let a = Farad::new(1.0);
        let b = 2.0;
        assert_eq!(a / b, Farad::new(0.5));
    }

    #[test]
    fn farad_div_assign_f64() {
        let mut a = Farad::new(1.0);
        let b = 2.0;
        a /= b;
        assert_eq!(a, Farad::new(0.5));
    }

    #[test]
    fn farad_div_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a / b, Farad::new(0.5));
    }

    #[test]
    fn farad_div_assign_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a /= b;
        assert_eq!(a, Farad::new(0.5));
    }

    #[test]
    fn farad_rem_f64() {
        let a = Farad::new(1.0);
        let b = 2.0;
        assert_eq!(a % b, Farad::new(1.0));
    }

    #[test]
    fn farad_rem_assign_f64() {
        let mut a = Farad::new(1.0);
        let b = 2.0;
        a %= b;
        assert_eq!(a, Farad::new(1.0));
    }

    #[test]
    fn farad_rem_farad() {
        let a = Farad::new(1.0);
        let b = Farad::new(2.0);
        assert_eq!(a % b, Farad::new(1.0));
    }

    #[test]
    fn farad_rem_assign_farad() {
        let mut a = Farad::new(1.0);
        let b = Farad::new(2.0);
        a %= b;
        assert_eq!(a, Farad::new(1.0));
    }

    #[test]
    fn farad_display() {
        let a = Farad::new(1.0);
        assert_eq!(format!("{a}"), "1 F");
    }

    #[test]
    fn farad_display_micro() {
        let a = Farad::new_microfarad(1.0);
        assert_eq!(format!("{a}"), "1 µF");
    }

    #[test]
    fn farad_reciprocal() {
        let a = Farad::new(1.0);
        assert_eq!(a.reciprocal(), Farad::new(1.0 / 1.0));
    }
}