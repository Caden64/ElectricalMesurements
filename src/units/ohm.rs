use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug)]
pub struct Ohm {
    pub value: f64,
}

impl Ohm {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Ohm> for Ohm {
    type Output = Ohm;

    fn mul(self, other: Ohm) -> Ohm {
        Ohm::new(self.value * other.value)
    }
}

impl MulAssign<Ohm> for Ohm {
    fn mul_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value * other.value);
    }
}

impl Div<Ohm> for Ohm {
    type Output = Ohm;

    fn div(self, other: Ohm) -> Ohm {
        Ohm::new(self.value / other.value)
    }
}

impl DivAssign<Ohm> for Ohm {
    fn div_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value / other.value);
    }
}

impl Add<Ohm> for Ohm {
    type Output = Ohm;

    fn add(self, other: Ohm) -> Ohm {
        Ohm::new(self.value + other.value)
    }
}

impl AddAssign<Ohm> for Ohm {
    fn add_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value + other.value);
    }
}

impl Sub<Ohm> for Ohm {
    type Output = Ohm;

    fn sub(self, other: Ohm) -> Ohm {
        Ohm::new(self.value - other.value)
    }
}

impl SubAssign<Ohm> for Ohm {
    fn sub_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value - other.value);
    }
}

impl Rem<Ohm> for Ohm {
    type Output = Ohm;

    fn rem(self, other: Ohm) -> Ohm {
        Ohm::new(self.value % other.value)
    }
}

impl RemAssign<Ohm> for Ohm {
    fn rem_assign(&mut self, other: Ohm) {
        *self = Ohm::new(self.value % other.value);
    }
}