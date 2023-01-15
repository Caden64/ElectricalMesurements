use crate::units::ohm::Ohm;

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn series_value_test() {
        let resistors = vec![Ohm::new(1.0), Ohm::new(2.0), Ohm::new(3.0)];
        let series = Series::new(resistors);
        assert_eq!(series.total_resistance(), Ohm::new(6.0))
    }
}