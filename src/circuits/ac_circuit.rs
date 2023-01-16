use crate::units::amp::Amp;
use crate::units::farad::Farad;
use crate::units::hertz::Hertz;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;
use crate::units::Watt::Watt;

struct Capacitive {
    hertz: Hertz,
    voltage: Volt,
    total_capacitance: Farad,
}

impl Capacitive {
    fn new(hertz: Hertz, voltage: Volt, total_capacitance: Farad) -> Capacitive {
        Capacitive {
            hertz,
            voltage,
            total_capacitance,
        }
    }

    fn reactance(&self) -> Ohm {
        self.hertz * self.total_capacitance
    }

    fn current(&self) -> Amp {
        self.voltage / self.reactance()
    }
    
}