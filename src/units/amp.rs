use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug)]
pub struct Amp {
    pub value: f64,
}

impl Amp {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Amp> for Amp {
    type Output = Amp;

    fn mul(self, other: Amp) -> Amp {
        Amp::new(self.value * other.value)
    }
}

impl MulAssign<Amp> for Amp {
    fn mul_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value * other.value);
    }
}

impl Div<Amp> for Amp {
    type Output = Amp;

    fn div(self, other: Amp) -> Amp {
        Amp::new(self.value / other.value)
    }
}

impl DivAssign<Amp> for Amp {
    fn div_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value / other.value);
    }
}

impl Add<Amp> for Amp {
    type Output = Amp;

    fn add(self, other: Amp) -> Amp {
        Amp::new(self.value + other.value)
    }
}

impl AddAssign<Amp> for Amp {
    fn add_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value + other.value);
    }
}

impl Sub<Amp> for Amp {
    type Output = Amp;

    fn sub(self, other: Amp) -> Amp {
        Amp::new(self.value - other.value)
    }
}

impl SubAssign<Amp> for Amp {
    fn sub_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value - other.value);
    }
}

impl Rem<Amp> for Amp {
    type Output = Amp;

    fn rem(self, other: Amp) -> Amp {
        Amp::new(self.value % other.value)
    }
}

impl RemAssign<Amp> for Amp {
    fn rem_assign(&mut self, other: Amp) {
        *self = Amp::new(self.value % other.value);
    }
}
