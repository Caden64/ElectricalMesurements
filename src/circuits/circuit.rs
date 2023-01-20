use crate::circuits::components::Components;
use crate::circuits::parallel::Parallel;
use crate::circuits::series::Series;
use crate::units::volt::Volt;

#[derive(PartialEq, Debug, Clone)]
pub enum CircuitSteps {
    Series(Series),
    Parallel(Parallel),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Circuit {
    pub steps: Vec<CircuitSteps>,
}

impl Circuit {
    pub fn from(steps: Vec<CircuitSteps>) -> Circuit {
        Circuit { steps }
    }
    pub fn new_series(component: Series) -> Circuit {
        Circuit {
            steps: vec![CircuitSteps::Series(component)],
        }
    }
    pub fn new_parallel(component: Parallel) -> Circuit {
        Circuit {
            steps: vec![CircuitSteps::Parallel(component)],
        }
    }

    pub fn new_voltage(voltage: Volt) -> Circuit {
        let steps = vec![CircuitSteps::Series(Series::new(Components::from_voltage(voltage)))];
        Circuit { steps }
    }

    pub fn append_series(&mut self, component: Series) {
        self.steps.push(CircuitSteps::Series(component));
    }

    pub fn append_parallel(&mut self, component: Parallel) {
        self.steps.push(CircuitSteps::Parallel(component));
    }
}

impl CircuitSteps {
    pub fn new_series(component: Components) -> CircuitSteps {
        CircuitSteps::Series(Series::new(component))
    }
    pub fn new_parallel(side1: Vec<Series>, side2: Vec<Series>) -> CircuitSteps {
        CircuitSteps::Parallel(Parallel::new(side1, side2))
    }
}

#[cfg(test)]
mod tests {
    use crate::units::ohm::Ohm;
    use super::*;

    #[test]
    fn test_new() {
        let series = CircuitSteps::new_series(Components::from_resistor(Ohm::new(1.0)));
        let parallel = CircuitSteps::new_parallel(vec![], vec![]);
        let circuit = Circuit::from(vec![series, parallel]);
        assert_eq!(
            circuit,
            Circuit {
                steps: vec![
                    CircuitSteps::Series(Series::new(Components::from_resistor(Ohm::new(1.0)))),
                    CircuitSteps::Parallel(Parallel::new(vec![], vec![]))
                ]
            }
        );
    }
}