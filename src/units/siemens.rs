use crate::units::ohm::Ohm;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Siemens {
    value: f64
}

impl Siemens {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

pub fn ohm_to_siemens(ohm: Ohm) -> Siemens {
    Siemens::new(1.0 / ohm.value)
}

pub fn siemens_to_ohm(siemens: Siemens) -> Ohm {
    Ohm::new(1.0 / siemens.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ohm_to_siemens() {
        assert_eq!(ohm_to_siemens(Ohm::new(1.0)), Siemens::new(1.0))
    }

    #[test]
    fn test_siemens_to_ohm() {
        assert_eq!(siemens_to_ohm(Siemens::new(1.0)), Ohm::new(1.0))
    }
}