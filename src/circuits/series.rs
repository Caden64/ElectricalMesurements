use crate::circuits::components::Components;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum UniSeries {
    Component(Components),
}

impl UniSeries {
    pub fn new(component: Components) -> UniSeries {
        UniSeries::Component(component)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::farad::Farad;
    use crate::units::henry::Henry;
    use crate::units::ohm::Ohm;
    use crate::units::volt::Volt;

    #[test]
    fn test_new() {
        let resistor = UniSeries::new(Components::new_resistor(Ohm::new(1.0)));
        let capacitor = UniSeries::new(Components::new_capacitor(Farad::new(1.0)));
        let inductor = UniSeries::new(Components::new_inductor(Henry::new(1.0)));
        let voltage = UniSeries::new(Components::new_voltage(Volt::new(1.0)));
        assert_eq!(resistor, UniSeries::Component(Components::Resistor(Ohm::new(1.0))));
        assert_eq!(capacitor, UniSeries::Component(Components::Capacitor(Farad::new(1.0))));
        assert_eq!(inductor, UniSeries::Component(Components::Inductor(Henry::new(1.0))));
        assert_eq!(voltage, UniSeries::Component(Components::Voltage(Volt::new(1.0))));
    }
}