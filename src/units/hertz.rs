 use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hertz {
    pub value: f64,
}

impl Hertz {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Mul<Hertz> for Hertz {
    type Output = Hertz;

    fn mul(self, other: Hertz) -> Hertz {
        Hertz::new(self.value * other.value)
    }
}

impl MulAssign<Hertz> for Hertz {
    fn mul_assign(&mut self, other: Hertz) {
        self.value *= other.value;
    }
}

impl Div<Hertz> for Hertz {
    type Output = Hertz;

    fn div(self, other: Hertz) -> Hertz {
        Hertz::new(self.value / other.value)
    }
}

impl DivAssign<Hertz> for Hertz {
    fn div_assign(&mut self, other: Hertz) {
        self.value /= other.value;
    }
}

impl Add<Hertz> for Hertz {
    type Output = Hertz;

    fn add(self, other: Hertz) -> Hertz {
        Hertz::new(self.value + other.value)
    }
}

impl AddAssign<Hertz> for Hertz {
    fn add_assign(&mut self, other: Hertz) {
        self.value += other.value;
    }
}

impl Sub<Hertz> for Hertz {
    type Output = Hertz;

    fn sub(self, other: Hertz) -> Hertz {
        Hertz::new(self.value - other.value)
    }
}

impl SubAssign<Hertz> for Hertz {
    fn sub_assign(&mut self, other: Hertz) {
        self.value -= other.value;
    }
}

 mod tests {
     use super::*;

     #[test]
     fn test_mul() {
            let a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            let c = a * b;
            assert_eq!(c.value, 2.0);
     }

        #[test]
        fn test_mul_assign() {
            let mut a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            a *= b;
            assert_eq!(a.value, 2.0);
        }

        #[test]
        fn test_div() {
            let a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            let c = a / b;
            assert_eq!(c.value, 0.5);
        }

        #[test]
        fn test_div_assign() {
            let mut a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            a /= b;
            assert_eq!(a.value, 0.5);
        }

        #[test]
        fn test_add() {
            let a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            let c = a + b;
            assert_eq!(c.value, 3.0);
        }

        #[test]
        fn test_add_assign() {
            let mut a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            a += b;
            assert_eq!(a.value, 3.0);
        }

        #[test]
        fn test_sub() {
            let a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            let c = a - b;
            assert_eq!(c.value, -1.0);
        }

        #[test]
        fn test_sub_assign() {
            let mut a = Hertz::new(1.0);
            let b = Hertz::new(2.0);
            a -= b;
            assert_eq!(a.value, -1.0);
        }
 }