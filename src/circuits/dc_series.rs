use crate::circuits::dc_math::{give_amp, give_voltage};
use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;

pub struct Series {
    pub resistors: Vec<Ohm>,
}

impl Series {
    pub fn new(resistors: Vec<Ohm>) -> Series {
        Series { resistors }
    }

    pub fn total_resistance(&self) -> Ohm {
        let mut total_resistance = Ohm::new(0.0);
        for resistor in &self.resistors {
            total_resistance += *resistor;
        }
        total_resistance
    }

    pub fn index_ohm(&self, index: usize) -> Ohm {
        for (i, resistor) in self.resistors.iter().enumerate() {
            if i == index {
                return *resistor;
            }
        }
        Ohm::new(0.0)
    }

    pub fn index_amp(&self, index: usize, input_voltage: Volt) -> Amp {
        let resistance = &self.index_ohm(index);
        if resistance != &Ohm::new(0.0) {
            return give_amp(input_voltage, *resistance)
        }
        Amp::new(0.0)
    }

    pub fn index_voltage(&self, index: usize, current: Amp) -> Volt {
        let resistance = &self.index_ohm(index);
        if resistance != &Ohm::new(0.0) {
            return give_voltage(current, *resistance)
        }
        Volt::new(0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn series_value_resistance_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.total_resistance(), Ohm::new(6.0))
    }

    #[test]
    fn series_index_ohm_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.index_ohm(0), Ohm::new(1.0))
    }

    #[test]
    fn series_index_amp_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.index_amp(0, Volt::new(1.0)), Amp::new(1.0))
    }

    #[test]
    fn series_index_voltage_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.index_voltage(0, Amp::new(1.0)), Volt::new(1.0))
    }
}