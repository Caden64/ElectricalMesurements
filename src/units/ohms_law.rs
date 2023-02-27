use std::ops::{Mul, Div};
use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;
use crate::units::watt::Watt;

// I = V / R
impl Div<Ohm> for Volt {
    type Output = Amp;

    fn div(self, other: Ohm) -> Amp {
        Amp::new(self.value / other.value)
    }
}

impl Div<Volt> for Ohm {
    type Output = Amp;

    fn div(self, other: Volt) -> Amp {
        Amp::new(other.value / self.value)
    }
}
// E = R * I
impl Mul<Ohm> for Amp {
    type Output = Volt;

    fn mul(self, other: Ohm) -> Volt {
        Volt::new(self.value * other.value)
    }
}

impl Mul<Amp> for Ohm {
    type Output = Volt;

    fn mul(self, other: Amp) -> Volt {
        Volt::new(other.value * self.value)
    }
}

// P = I*E

impl Mul<Volt> for Amp {
    type Output = Watt;

    fn mul(self, other: Volt) -> Watt {
        Watt::new(self.value * other.value)
    }
}

impl Mul<Amp> for Volt {
    type Output = Watt;

    fn mul(self, other: Amp) -> Watt {
        Watt::new(other.value * self.value)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_volt_div_ohm() {
        let v = Volt::new(1.0);
        let r = Ohm::new(1.0);
        let i = v / r;
        assert_eq!(i.value, 1.0);
    }

    #[test]
    fn test_ohm_div_volt() {
        let v = Volt::new(1.0);
        let r = Ohm::new(1.0);
        let i = r / v;
        assert_eq!(i.value, 1.0);
    }

    #[test]
    fn test_ohm_mul_amp() {
        let r = Ohm::new(1.0);
        let i = Amp::new(1.0);
        let v = r * i;
        assert_eq!(v.value, 1.0);
    }

    #[test]
    fn test_amp_mul_ohm() {
        let r = Ohm::new(1.0);
        let i = Amp::new(1.0);
        let v = i * r;
        assert_eq!(v.value, 1.0);
    }

    #[test]
    fn test_volt_mul_amp() {
        let v = Volt::new(1.0);
        let i = Amp::new(1.0);
        let p = v * i;
        assert_eq!(p.value, 1.0);
    }

    #[test]
    fn test_amp_mul_volt() {
        let v = Volt::new(1.0);
        let i = Amp::new(1.0);
        let p = i * v;
        assert_eq!(p.value, 1.0);
    }
}