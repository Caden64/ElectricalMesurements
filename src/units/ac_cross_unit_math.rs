use std::ops::Mul;
use crate::units::farad::Farad;
use crate::units::hertz::Hertz;
use crate::units::ohm::Ohm;
use std::f64::consts::PI;
use crate::units::henry::Henry;

impl Mul<Hertz> for Farad {
    type Output = Ohm;

    fn mul(self, other: Hertz) -> Ohm {
        Ohm::new(1.0 / (2.0 * PI * self.value * other.value))
    }
}

impl Mul<Farad> for Hertz {
    type Output = Ohm;

    fn mul(self, other: Farad) -> Ohm {
        Ohm::new(1.0 / (2.0 * PI * self.value * other.value))
    }
}

impl Mul<Farad> for Henry {
    type Output = Hertz;

    fn mul(self, other: Farad) -> Hertz {
        Hertz::new(2.0 * PI * self.value * other.value)
    }
}

impl Mul<Henry> for Farad {
    type Output = Hertz;

    fn mul(self, other: Henry) -> Hertz {
        Hertz::new(2.0 * PI * self.value * other.value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_h_f_mul_eq_ohm() {
        assert_eq!(Hertz::new(1.0) * Farad::new(1.0), Ohm::new(1.0 / (2.0 * PI)))
    }

    #[test]
    fn test_f_h_mul_eq_ohm() {
        assert_eq!(Farad::new(1.0) * Hertz::new(1.0), Ohm::new(1.0 / (2.0 * PI)))
    }

    #[test]
    fn example_f_g_mul_eq_ohm() {
        assert_eq!(Farad::new_microfarad(0.27) * Hertz::new(200.0), Ohm::new(2947.313760961025))
    }
}