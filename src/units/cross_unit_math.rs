use std::ops::{Div, Mul};
use crate::units::Amp::Amp;
use crate::units::Ohm::Ohm;
use crate::units::Volt::Volt;

// Ohm

// Ohm * Volt = Amp
impl Mul<Volt> for Ohm {
    type Output = Amp;

    fn mul(self, other: Volt) -> Amp {
        Amp::new(other.value / self.value)
    }
}

// Ohm * Amp = Volt
impl Mul<Amp> for Ohm {
    type Output = Volt;

    fn mul(self, other: Amp) -> Volt {
        Volt::new(self.value * other.value)
    }
}

// Volt / Amp = Ohm
impl Div<Amp> for Volt {
    type Output = Ohm;

    fn div(self, other: Amp) -> Ohm {
        Ohm::new(self.value / other.value)
    }
}

// Volt

// Volt * Ohm = Amp
impl Mul<Ohm> for Volt {
    type Output = Amp;

    fn mul(self, other: Ohm) -> Amp {
        Amp::new(self.value / other.value)
    }
}
// Volt * Amp = Ohm
impl Mul<Amp> for Volt {
    type Output = Ohm;

    fn mul(self, other: Amp) -> Ohm {
        Ohm::new(self.value / other.value)
    }
}
// Amp

// Amp * Ohm = Volt
impl Mul<Ohm> for Amp {
    type Output = Volt;

    fn mul(self, other: Ohm) -> Volt {
        Volt::new(self.value * other.value)
    }
}

// Amp * Volt = Ohm
impl Mul<Volt> for Amp {
    type Output = Ohm;

    fn mul(self, other: Volt) -> Ohm {
        Ohm::new(other.value / self.value)
    }
}

// Amp = Volt / Ohm
impl Div<Ohm> for Volt {
    type Output = Amp;

    fn div(self, other: Ohm) -> Amp {
        Amp::new(self.value / other.value)
    }
}