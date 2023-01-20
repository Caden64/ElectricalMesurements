use crate::circuits::components::Components;
use crate::circuits::parallel::UniParallel;
use crate::circuits::series::UniSeries;

#[derive(PartialEq, Debug, Clone)]
pub enum CircuitSteps {
    Series(UniSeries),
    Parallel(UniParallel),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Circuit {
    pub steps: Vec<CircuitSteps>,
}

impl Circuit {
    pub fn from(steps: Vec<CircuitSteps>) -> Circuit {
        Circuit { steps }
    }
    pub fn new_series(component: UniSeries) -> Circuit {
        Circuit {
            steps: vec![CircuitSteps::Series(component)],
        }
    }
    pub fn new_parallel(component: UniParallel) -> Circuit {
        Circuit {
            steps: vec![CircuitSteps::Parallel(component)],
        }
    }
}

impl CircuitSteps {
    pub fn new_series(component: Components) -> CircuitSteps {
        CircuitSteps::Series(UniSeries::new(component))
    }
    pub fn new_parallel(components: Vec<UniSeries>) -> CircuitSteps {
        CircuitSteps::Parallel(UniParallel::new(components))
    }
}

#[cfg(test)]
mod tests {
    use crate::units::ohm::Ohm;
    use super::*;

    #[test]
    fn test_new() {
        let series = CircuitSteps::new_series(Components::from_resistor(Ohm::new(1.0)));
        let parallel = CircuitSteps::new_parallel(vec![]);
        let circuit = Circuit::from(vec![series, parallel]);
        assert_eq!(
            circuit,
            Circuit {
                steps: vec![
                    CircuitSteps::Series(UniSeries::new(Components::from_resistor(Ohm::new(1.0)))),
                    CircuitSteps::Parallel(UniParallel::new(vec![]))
                ]
            }
        );
    }
}