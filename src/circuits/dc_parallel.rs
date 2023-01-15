use crate::circuits::dc_math::{give_amp, give_voltage};
use crate::circuits::dc_series::Series;
use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;

pub struct Parallel {
    pub left_resistors: Series,
    pub right_resistors: Series,
}

impl Parallel {
    pub fn new(left_resistors: Series, right_resistors: Series) -> Parallel {
        Parallel {
            left_resistors,
            right_resistors,
        }
    }

    pub fn total_resistance(&self) -> Ohm {
        let left_resistance = self.left_resistors.total_resistance();
        let right_resistance = self.right_resistors.total_resistance();
        let mut total_resistance = left_resistance * right_resistance;
        total_resistance /= left_resistance + right_resistance;
        total_resistance
    }

    pub fn index_ohm(&self, index: usize, left_side: bool) -> Ohm {
        if left_side {
            for (i, resistor) in self.left_resistors.resistors.iter().enumerate() {
                if i == index {
                    return *resistor;
                }
            }
        } else {
            for (i, resistor) in self.right_resistors.resistors.iter().enumerate() {
                if i == index {
                    return *resistor;
                }
            }
        }
        Ohm::new(0.0)
    }

    pub fn index_amp(&self, index: usize, input_voltage: Volt ,left_side: bool) -> Amp {
        let resistance = &self.index_ohm(index, left_side);
        if resistance != &Ohm::new(0.0) {
            return give_amp(input_voltage, *resistance)
        }
        Amp::new(0.0)
    }

    pub fn index_voltage(&self, index: usize, current: Amp, left_side: bool) -> Volt {
        let resistance = &self.index_ohm(index, left_side);
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
    fn parallel_small_values_test() {
        let left_resistors = Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        let right_resistors = Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        let parallel = Parallel::new(left_resistors, right_resistors);
        assert_eq!(parallel.total_resistance(), Ohm::new(3.0))
    }

    #[test]
    fn parallel_real_values_test() {
        let left_resistors = Series::new(vec![Ohm::new(220.0)]);
        let right_resistors = Series::new(vec![Ohm::new(100.0)]);
        let parallel = Parallel::new(left_resistors, right_resistors);
        assert_eq!(Ohm::new(parallel.total_resistance().value.round()), Ohm::new(69.0))
    }
}