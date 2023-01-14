#[macro_export]
macro_rules! measure {
        ($id:ident, $symbol:literal) => {
            #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
            pub struct $id(pub f64);

            impl fmt::Display for $id {
                fn fmt(&self, sdt: &mut fmt::Formatter) -> fmt::Result {
                    write!(sdt, "{:.2} {}", self.0, $symbol)
                }
            }

            impl Add for $id {
               type Output = $id;

               fn add(self, other: $id) -> $id {
                   $id (self.0 + other.0)
               }
            }

            impl AddAssign for $id {
                fn add_assign(&mut self, other: $id) {
                    *self = $id (self.0 + other.0);
                }
            }

            impl Sub for $id {
                type Output = $id;

                fn sub(self, other: $id) -> $id {
                    $id(self.0 - other.0)
                }
            }

            impl SubAssign for $id {
                fn sub_assign(&mut self, other: $id) {
                    *self = $id(self.0 - other.0);
                }
            }

            impl Mul for $id {
                type Output = $id;

                fn mul(self, other: $id) -> $id {
                    $id(self.0 * other.0)
                }
            }

            impl MulAssign for $id {
                fn mul_assign(&mut self, other: $id) {
                    *self = $id(self.0 * other.0);
                }
            }

            impl Div for $id {
                type Output = $id;

                fn div(self, other: $id) -> $id {
                    $id(self.0 / other.0)
                }
            }

            impl DivAssign for $id {
                fn div_assign(&mut self, other: $id) {
                    *self = $id(self.0 / other.0);
                }
            }
        }
    }
pub(crate) use measure;