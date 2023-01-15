use std::env::current_exe;
use crate::circuits::dc_math::{give_amp, give_voltage};
use crate::circuits::dc_parallel::Parallel;
use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;
use crate::circuits::dc_series::Series;

enum CircuitSteps {
    Series(Series),
    Parallel(Parallel),
}

pub struct Circuit {
    input_voltage: Volt,
    steps: Vec<CircuitSteps>,
}

impl Circuit {
    pub fn new(input_voltage: Volt) -> Circuit {
        Circuit {
            input_voltage,
            steps: Vec::new(),
        }
    }

    pub fn add_series(&mut self, resistors: Vec<Ohm>) {
        self.steps.push(CircuitSteps::Series(Series::new(resistors)));
    }

    pub fn add_parallel(&mut self, left_resistors: Series, right_resistors: Series) {
        self.steps
            .push(CircuitSteps::Parallel(Parallel::new(left_resistors, right_resistors)));
    }

    pub fn total_resistance(&self) -> Ohm {
        let mut total_resistance = Ohm::new(0.0);
        for step in &self.steps {
            match step {
                CircuitSteps::Series(series) => total_resistance += series.total_resistance(),
                CircuitSteps::Parallel(parallel) => total_resistance += parallel.total_resistance(),
            }
        }
        total_resistance
    }

    pub fn total_amp(&self) -> Amp {
        give_amp(self.input_voltage, self.total_resistance())
    }

    pub fn index_ohm(&self, index: usize) -> Ohm {
        for (i, step) in self.steps.iter().enumerate() {
            if i == index {
                return match step {
                    CircuitSteps::Series(series) => series.total_resistance(),
                    CircuitSteps::Parallel(parallel) => parallel.total_resistance(),
                }
            }
        }
        Ohm::new(0.0)
    }

    pub fn index_amp(&self, index: usize) -> Amp {
        let resistance = &self.index_ohm(index);
        if resistance != &Ohm::new(0.0) {
            return give_amp(self.input_voltage, *resistance)
        }
        Amp::new(0.0)
    }

    pub fn index_voltage(&self, index: usize) -> Volt {
        let resistance = self.index_ohm(index);
        if resistance != Ohm::new(0.0) && index < self.steps.len(){
            return match &self.steps[index] {
                CircuitSteps::Series(s) => {
                    give_voltage(self.index_amp(index), s.total_resistance())
                },
                CircuitSteps::Parallel(p) => {
                    give_voltage(self.index_amp(index), p.total_resistance())
                },
            }
        }
        Volt::new(0.0)
    }

    pub fn index_sub_ohm(&self, index: usize, sub_index: usize, left_side: bool) -> Ohm {
        if index < self.steps.len() {
            return match &self.steps[index] {
                CircuitSteps::Series(s) => s.index_ohm(sub_index),
                CircuitSteps::Parallel(p) => p.index_ohm(sub_index, left_side),
            }
        }
        Ohm::new(0.0)
    }

    pub fn index_sub_amp(&self, index: usize, sub_index: usize, left_side: bool) -> Amp {
        if index < self.steps.len() {
            return match &self.steps[index] {
                CircuitSteps::Series(s) => s.index_amp(sub_index, self.input_voltage),
                CircuitSteps::Parallel(p) => p.index_amp(sub_index, self.input_voltage, left_side),
            }
        }
        Amp::new(0.0)
    }

    pub fn index_sub_voltage(&self, index: usize, sub_index: usize, current: Amp, left_side: bool) -> Volt {
        if index < self.steps.len() {
            return match &self.steps[index] {
                CircuitSteps::Series(s) => s.index_voltage(sub_index, current),
                CircuitSteps::Parallel(p) => p.index_voltage(sub_index, current, left_side),
            }
        }
        Volt::new(0.0)
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_series_resistance_test() {
        let mut circuit = Circuit::new(Volt::new(10.0));
        circuit.add_series(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        assert_eq!(circuit.total_resistance(), Ohm::new(6.0))
    }

    #[test]
    fn circuit_series_voltage_test() {
        let mut circuit = Circuit::new(Volt::new(10.0));
        circuit.add_series(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        assert_eq!(circuit.index_voltage(0), Volt::new(10.0))
    }

    #[test]
    fn circuit_parallel_resistance_test() {
        let mut circuit = Circuit::new(Volt::new(10.0));
        circuit.add_parallel(
            Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]),
            Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]),
        );
        assert_eq!(circuit.total_resistance(), Ohm::new(3.0))
    }

    #[test]
    fn circuit_combination_resistance_test_1() {
        let mut circuit = Circuit::new(Volt::new(10.0));
        circuit.add_series(vec![Ohm::new(150.0)]);
        circuit.add_parallel(
            Series::new(vec![Ohm::new(220.0)]),
            Series::new(vec![Ohm::new(100.0)]),
        );
        assert_eq!(Ohm::new(circuit.total_resistance().value.round()), Ohm::new(219.0))
    }

    #[test]
    fn circuit_combination_resistance_test_2() {
        let mut circuit = Circuit::new(Volt::new(5.0));
        circuit.add_parallel(
            Series::new(vec![Ohm::new(470.0)]),
            Series::new(vec![Ohm::new(220.0), Ohm::new(330.0)]),
        );
        assert_eq!(Ohm::new(circuit.total_resistance().value.round()), Ohm::new(253.0))
    }

}