use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Watt {
    pub(crate) value: f64,
}

impl Watt {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn new_kilowatt(value: f64) -> Self {
        Self { value: value * 1e3 }
    }

    pub fn new_megawatt(value: f64) -> Self {
        Self { value: value * 1e6 }
    }

    pub fn new_gigawatt(value: f64) -> Self {
        Self { value: value * 1e9 }
    }
}

impl Mul<Watt> for Watt {
    type Output = Watt;

    fn mul(self, other: Watt) -> Watt {
        Watt::new(self.value * other.value)
    }
}

impl MulAssign<Watt> for Watt {
    fn mul_assign(&mut self, other: Watt) {
        *self = Watt::new(self.value * other.value);
    }
}

impl Div<Watt> for Watt {
    type Output = Watt;

    fn div(self, other: Watt) -> Watt {
        Watt::new(self.value / other.value)
    }
}

impl DivAssign<Watt> for Watt {
    fn div_assign(&mut self, other: Watt) {
        *self = Watt::new(self.value / other.value);
    }
}

impl Add<Watt> for Watt {
    type Output = Watt;

    fn add(self, other: Watt) -> Watt {
        Watt::new(self.value + other.value)
    }
}

impl AddAssign<Watt> for Watt {
    fn add_assign(&mut self, other: Watt) {
        *self = Watt::new(self.value + other.value);
    }
}

impl Sub<Watt> for Watt {
    type Output = Watt;

    fn sub(self, other: Watt) -> Watt {
        Watt::new(self.value - other.value)
    }
}

impl SubAssign<Watt> for Watt {
    fn sub_assign(&mut self, other: Watt) {
        *self = Watt::new(self.value - other.value);
    }
}

impl Rem<Watt> for Watt {
    type Output = Watt;

    fn rem(self, other: Watt) -> Watt {
        Watt::new(self.value % other.value)
    }
}

impl RemAssign<Watt> for Watt {
    fn rem_assign(&mut self, other: Watt) {
        *self = Watt::new(self.value % other.value);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn watt_eq_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        assert_eq!(watt1, watt2);
    }

    #[test]
    fn watt_mul_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        let watt3 = watt1 * watt2;
        assert_eq!(watt3, super::Watt::new(1.0));
    }

    #[test]
    fn watt_mul_assign_watt() {
        let mut watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        watt1 *= watt2;
        assert_eq!(watt1, super::Watt::new(1.0));
    }

    #[test]
    fn watt_div_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        let watt3 = watt1 / watt2;
        assert_eq!(watt3, super::Watt::new(1.0));
    }

    #[test]
    fn watt_div_assign_watt() {
        let mut watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        watt1 /= watt2;
        assert_eq!(watt1, super::Watt::new(1.0));
    }

    #[test]
    fn watt_add_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        let watt3 = watt1 + watt2;
        assert_eq!(watt3, super::Watt::new(2.0));
    }

    #[test]
    fn watt_add_assign_watt() {
        let mut watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        watt1 += watt2;
        assert_eq!(watt1, super::Watt::new(2.0));
    }

    #[test]
    fn watt_sub_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        let watt3 = watt1 - watt2;
        assert_eq!(watt3, super::Watt::new(0.0));
    }

    #[test]
    fn watt_sub_assign_watt() {
        let mut watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        watt1 -= watt2;
        assert_eq!(watt1, super::Watt::new(0.0));
    }

    #[test]
    fn watt_rem_watt() {
        let watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        let watt3 = watt1 % watt2;
        assert_eq!(watt3, super::Watt::new(0.0));
    }

    #[test]
    fn watt_rem_assign_watt() {
        let mut watt1 = super::Watt::new(1.0);
        let watt2 = super::Watt::new(1.0);
        watt1 %= watt2;
        assert_eq!(watt1, super::Watt::new(0.0));
    }

}
