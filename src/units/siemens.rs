use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};


use crate::units::ohm::Ohm;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Siemens {
    pub(crate) value: f64
}

impl Siemens {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Siemens> for Siemens {
    type Output = Siemens;

    fn mul(self, other: Siemens) -> Siemens {
        Siemens::new(self.value * other.value)
    }
}

impl MulAssign<Siemens> for Siemens {
    fn mul_assign(&mut self, other: Siemens) {
        *self = Siemens::new(self.value * other.value);
    }
}

impl Div<Siemens> for Siemens {
    type Output = Siemens;

    fn div(self, other: Siemens) -> Siemens {
        Siemens::new(self.value / other.value)
    }
}

impl DivAssign<Siemens> for Siemens {
    fn div_assign(&mut self, other: Siemens) {
        *self = Siemens::new(self.value / other.value);
    }
}

impl Add<Siemens> for Siemens {
    type Output = Siemens;

    fn add(self, other: Siemens) -> Siemens {
        Siemens::new(self.value + other.value)
    }
}

impl AddAssign<Siemens> for Siemens {
    fn add_assign(&mut self, other: Siemens) {
        *self = Siemens::new(self.value + other.value);
    }
}

impl Sub<Siemens> for Siemens {
    type Output = Siemens;

    fn sub(self, other: Siemens) -> Siemens {
        Siemens::new(self.value - other.value)
    }
}

impl SubAssign<Siemens> for Siemens {
    fn sub_assign(&mut self, other: Siemens) {
        *self = Siemens::new(self.value - other.value);
    }
}

impl Rem<Siemens> for Siemens {
    type Output = Siemens;

    fn rem(self, other: Siemens) -> Siemens {
        Siemens::new(self.value % other.value)
    }
}

impl RemAssign<Siemens> for Siemens {
    fn rem_assign(&mut self, other: Siemens) {
        *self = Siemens::new(self.value % other.value);
    }
}

pub fn ohm_to_siemens(ohm: Ohm) -> Siemens {
    Siemens::new(1.0 / ohm.value)
}

pub fn siemens_to_ohm(siemens: Siemens) -> Ohm {
    Ohm::new(1.0 / siemens.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ohm_to_siemens() {
        assert_eq!(ohm_to_siemens(Ohm::new(1.0)), Siemens::new(1.0))
    }

    #[test]
    fn test_siemens_to_ohm() {
        assert_eq!(siemens_to_ohm(Siemens::new(1.0)), Ohm::new(1.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Siemens::new(2.0) * Siemens::new(3.0), Siemens::new(6.0))
    }

    #[test]
    fn test_mul_assign() {
        let mut siemens = Siemens::new(2.0);
        siemens *= Siemens::new(3.0);
        assert_eq!(siemens, Siemens::new(6.0))
    }

    #[test]
    fn test_div() {
        assert_eq!(Siemens::new(6.0) / Siemens::new(3.0), Siemens::new(2.0))
    }

    #[test]
    fn test_div_assign() {
        let mut siemens = Siemens::new(6.0);
        siemens /= Siemens::new(3.0);
        assert_eq!(siemens, Siemens::new(2.0))
    }

    #[test]
    fn test_add() {
        assert_eq!(Siemens::new(2.0) + Siemens::new(3.0), Siemens::new(5.0))
    }

    #[test]
    fn test_add_assign() {
        let mut siemens = Siemens::new(2.0);
        siemens += Siemens::new(3.0);
        assert_eq!(siemens, Siemens::new(5.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(Siemens::new(5.0) - Siemens::new(3.0), Siemens::new(2.0))
    }

    #[test]
    fn test_sub_assign() {
        let mut siemens = Siemens::new(5.0);
        siemens -= Siemens::new(3.0);
        assert_eq!(siemens, Siemens::new(2.0))
    }

    #[test]
    fn test_rem() {
        assert_eq!(Siemens::new(5.0) % Siemens::new(3.0), Siemens::new(2.0))
    }

    #[test]
    fn test_rem_assign() {
        let mut siemens = Siemens::new(5.0);
        siemens %= Siemens::new(3.0);
        assert_eq!(siemens, Siemens::new(2.0))
    }

}