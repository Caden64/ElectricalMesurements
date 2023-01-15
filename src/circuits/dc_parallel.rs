use crate::circuits::dc_series::Series;
use crate::units::ohm::Ohm;

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