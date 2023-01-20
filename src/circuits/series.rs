use crate::circuits::components::Components;

#[derive(PartialEq, Debug, Clone)]
pub struct Series {
    components: Vec<Components>,
}

impl Series {
    pub fn new(component: Components) -> Series {
        Series {components: vec![component]}
    }

    pub fn add(&mut self, component: Components) {
        self.components.push(component)
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
        let resistor = Series::new(Components::from_resistor(Ohm::new(1.0)));
        let capacitor = Series::new(Components::from_capacitor(Farad::new(1.0)));
        let inductor = Series::new(Components::from_inductor(Henry::new(1.0)));
        let voltage = Series::new(Components::from_voltage(Volt::new(1.0)));
        assert_eq!(resistor, Series {components: vec![Components::Resistor(Ohm::new(1.0))]});
        assert_eq!(capacitor, Series {components: vec![Components::Capacitor(Farad::new(1.0))]});
        assert_eq!(inductor, Series {components: vec![Components::Inductor(Henry::new(1.0))]});
        assert_eq!(voltage, Series {components: vec![Components::Voltage(Volt::new(1.0))]});
    }
}