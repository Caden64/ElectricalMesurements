use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Rem, RemAssign};
#[derive(PartialEq, Debug)]
pub struct Volt {
    pub value: f64,
}

impl Volt {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Volt> for Volt {
    type Output = Volt;

    fn mul(self, other: Volt) -> Volt {
        Volt::new(self.value * other.value)
    }
}

impl MulAssign<Volt> for Volt {
    fn mul_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value * other.value);
    }
}

impl Div<Volt> for Volt {
    type Output = Volt;

    fn div(self, other: Volt) -> Volt {
        Volt::new(self.value / other.value)
    }
}

impl DivAssign<Volt> for Volt {
    fn div_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value / other.value);
    }
}

impl Add<Volt> for Volt {
    type Output = Volt;

    fn add(self, other: Volt) -> Volt {
        Volt::new(self.value + other.value)
    }
}

impl AddAssign<Volt> for Volt {
    fn add_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value + other.value);
    }
}

impl Sub<Volt> for Volt {
    type Output = Volt;

    fn sub(self, other: Volt) -> Volt {
        Volt::new(self.value - other.value)
    }
}

impl SubAssign<Volt> for Volt {
    fn sub_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value - other.value);
    }
}

impl Rem<Volt> for Volt {
    type Output = Volt;

    fn rem(self, other: Volt) -> Volt {
        Volt::new(self.value % other.value)
    }
}

impl RemAssign<Volt> for Volt {
    fn rem_assign(&mut self, other: Volt) {
        *self = Volt::new(self.value % other.value);
    }
}