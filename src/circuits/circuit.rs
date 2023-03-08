use std::collections::LinkedList;
use crate::units::farad::Farad;
use crate::units::henry::Henry;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;
use crate::circuits::voltage::VoltagePolarity;

#[derive(Debug, PartialEq)]
enum CircuitComponents {
    Voltage(Volt),
    Polarity(VoltagePolarity),
    Resistance(Ohm),
    Capacitors(Farad),
    Inductors(Henry),
}
impl Default for CircuitComponents {
    fn default() -> Self {
        CircuitComponents::Polarity(VoltagePolarity::default())
    }
}
trait CircuitComponent {}

struct SeriesCircuit {
    components: LinkedList<CircuitComponents>,
}

impl CircuitComponent for SeriesCircuit {}

struct ParallelCircuit {
    side1: SeriesCircuit,
    side2: SeriesCircuit,
}

impl CircuitComponent for ParallelCircuit {}

struct Circuit<T: CircuitComponent> {
    components: Vec<T>,
}
