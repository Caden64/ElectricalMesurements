use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;

pub fn give_amp(voltage: Volt, resistance: Ohm) -> Amp {
    voltage / resistance
}

pub fn give_voltage(amp: Amp, resistance: Ohm) -> Volt {
    amp * resistance
}

pub fn give_resistance(voltage: Volt, amp: Amp) -> Ohm {
    voltage / amp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_amp() {
        assert_eq!(give_amp(Volt::new(10.0), Ohm::new(10.0)), Amp::new(1.0))
    }

    #[test]
    fn test_give_voltage() {
        assert_eq!(give_voltage(Amp::new(10.0), Ohm::new(10.0)), Volt::new(100.0))
    }

    #[test]
    fn test_give_resistance() {
        assert_eq!(give_resistance(Volt::new(10.0), Amp::new(10.0)), Ohm::new(1.0))
    }
}