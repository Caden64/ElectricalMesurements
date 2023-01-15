use crate::units::amp::Amp;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;


pub struct Series {
    pub resistors: Vec<Ohm>,
}

pub struct Parallel {
    pub left_resistors: Series,
    pub right_resistors: Series,
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
}

enum CircuitSteps {
    Series(Series),
    Parallel(Parallel),
    Voltage(Volt),
    Amps(Amp),
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

    pub fn add_voltage(&mut self, voltage: Volt) {
        self.steps.push(CircuitSteps::Voltage(voltage));
    }

    pub fn add_amps(&mut self, amps: Amp) {
        self.steps.push(CircuitSteps::Amps(amps));
    }

    pub fn total_resistance(&self) -> Ohm {
        let mut total_resistance = Ohm::new(0.0);
        for step in &self.steps {
            match step {
                CircuitSteps::Series(series) => total_resistance += series.total_resistance(),
                CircuitSteps::Parallel(parallel) => total_resistance += parallel.total_resistance(),
                _ => {}
            }
        }
        total_resistance
    }

    pub fn total_voltage(&self) -> Volt {
        let mut total_voltage = Volt::new(0.0);
        for step in &self.steps {
            if let CircuitSteps::Voltage(voltage) = step { total_voltage += *voltage }
        }
        total_voltage
    }

    pub fn total_amps(&self) -> Amp {
        let mut total_amps = Amp::new(0.0);
        for step in &self.steps {
            if let CircuitSteps::Amps(amps) = step { total_amps += *amps }
        }
        total_amps
    }
/*
    pub fn total_power(&self) -> Watt {
        let total_voltage = self.total_voltage();
        let total_amps = self.total_amps();
        total_voltage * total_amps
    }

 */
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::units::ohm::Ohm;

    #[test]
    fn series_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.total_resistance(), Ohm::new(6.0))
    }

    #[test]
    fn parallel_test() {
        let left_resistors = Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        let right_resistors = Series::new(vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)]);
        let parallel = Parallel::new(left_resistors, right_resistors);
        assert_eq!(parallel.total_resistance(), Ohm::new(3.0))
    }
}