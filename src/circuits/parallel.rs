use crate::circuits::components::Components;
use crate::circuits::series::Series;
use crate::units::amp::Amp;
use crate::units::farad::Farad;
use crate::units::ohm::Ohm;

#[derive(PartialEq, Debug, Clone)]
pub struct Parallel {
    pub side1: Vec<Series>,
    pub side2: Vec<Series>
}

impl Parallel {
    pub fn new(side1: Vec<Series>, side2: Vec<Series>) -> Parallel {
        Parallel { side1, side2 }
    }

    pub fn capacitance(&self) -> Farad {
        let mut all_capacitors = Farad::new(0.0);

        for capacitors in &self.side1 {
            for capacitor in &capacitors.components {
                match capacitor {
                    Components::Capacitor(farad) => {
                        all_capacitors += *farad;
                    },
                    _ => {}
                }
            }
        }

        for capacitors in &self.side2 {
            for capacitor in &capacitors.components {
                match capacitor {
                    Components::Capacitor(farad) => {
                        all_capacitors += *farad;
                    },
                    _ => {}
                }
            }
        }

        all_capacitors
    }
    pub fn total_resistance(&self) -> Ohm {
        let mut total = Ohm::new(0.0);
        for series in &self.side1 {
            for component in &series.components {
                match component {
                    Components::Resistor(ohm) => {
                        total += ohm.reciprocal()
                    }
                    _ => {}
                }
            }
        }

        for series in &self.side2 {
            for component in &series.components {
                match component {
                    Components::Resistor(ohm) => {
                        total += ohm.reciprocal()
                    }
                    _ => {}
                }
            }
        }

        total.reciprocal()
    }
    pub fn total_amps(&self) -> Amp {
        let mut total = Amp::new(0.0);
        for x in &self.side1 {
            total += x.total_amps()
        }

        for x in &self.side2 {
            total += x.total_amps()
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::circuits::parallel::Parallel;
    use crate::circuits::series::Series;
    use crate::units::farad::Farad;

    #[test]
    fn test_new() {
        let side1 = vec![];
        let side2 = vec![];
        let parallel = Parallel::new(side1, side2);
        assert_eq!(parallel, Parallel { side1: vec![], side2: vec![] });
    }

    #[test]
    fn capacitance_test() {
        let side1 = vec![Series::from_capacitor(Farad::new_microfarad(0.27))];
        let side2  = vec![Series::from_capacitor(Farad::new_microfarad(0.27))];
        let parallel = Parallel::new(side1, side2);
        assert_eq!(parallel.capacitance(), Farad::new_microfarad(0.54))
    }
}