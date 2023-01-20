use crate::circuits::components::Components;
use crate::circuits::series::Series;
use crate::units::farad::Farad;

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