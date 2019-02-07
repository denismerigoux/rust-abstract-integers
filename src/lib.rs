extern crate num;
#[allow(unused_imports)]
use num::{BigUint, CheckedSub, Zero};
use std::ops::*;

macro_rules! define_abstract_integer_bounded_operations {
    ($name:ident, $bytes:literal) => {
        impl Add for $name {
            type Output = $name;
            fn add(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c = a + b;
                if c > $name::max() {
                    panic!("bounded addition overflow for type {}", stringify!($name));
                }
                c.into()
            }
        }

        impl Sub for $name {
            type Output = $name;
            fn sub(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c = a.checked_sub(&b).unwrap_or_else(|| {
                    panic!(
                        "bounded substraction underflow for type {}",
                        stringify!($name)
                    )
                });
                c.into()
            }
        }

        impl Mul for $name {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c = a * b;
                if c > $name::max() {
                    panic!("bounded addition overflow for type {}", stringify!($name));
                }
                c.into()
            }
        }

        impl Div for $name {
            type Output = $name;
            fn div(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                if b == BigUint::zero() {
                    panic!("dividing by zero in type {}", stringify!($name));
                }
                let c = a / b;
                c.into()
            }
        }

        impl Rem for $name {
            type Output = $name;
            fn rem(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                if b == BigUint::zero() {
                    panic!("dividing by zero in type {}", stringify!($name));
                }
                let c = a % b;
                c.into()
            }
        }
    };
}

macro_rules! define_abstract_integer_modular_operations {
    ($name:ident, $bytes:literal) => {
        impl Add for $name {
            type Output = $name;
            fn add(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a + b;
                let d: BigUint = c % $name::max();
                d.into()
            }
        }

        impl Sub for $name {
            type Output = $name;
            fn sub(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = if b > a { $name::max() - b + a } else { b - a };
                c.into()
            }
        }

        impl Mul for $name {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a * b;
                let d: BigUint = c % $name::max();
                d.into()
            }
        }

        impl Div for $name {
            type Output = $name;
            fn div(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                if b == BigUint::zero() {
                    panic!("dividing by zero in type {}", stringify!($name));
                }
                let c: BigUint = a / b;
                c.into()
            }
        }

        impl Rem for $name {
            type Output = $name;
            fn rem(self, rhs: $name) -> $name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                if b == BigUint::zero() {
                    panic!("dividing by zero in type {}", stringify!($name));
                }
                let c: BigUint = a % b;
                c.into()
            }
        }
    };
}

macro_rules! define_abstract_integer_struct {
    ($name:ident, $bytes:literal, $max:expr) => {
        /// Little endian byte representation of the integer
        #[derive(Clone, Copy)]
        pub struct $name([u8; $bytes]);

        impl From<BigUint> for $name {
            fn from(x: BigUint) -> $name {
                let repr = x.to_bytes_be();
                if repr.len() > $bytes {
                    panic!("BigUint too big for type {}", stringify!($name))
                }
                let mut out = [0u8; $bytes];
                let upper = out.len();
                let lower = upper - repr.len();
                out[lower..upper].copy_from_slice(&repr);
                $name(out)
            }
        }

        impl Into<BigUint> for $name {
            fn into(self) -> BigUint {
                BigUint::from_bytes_be(&self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let uint: BigUint = (*self).into();
                write!(f, "{}", uint)
            }
        }

        impl $name {
            fn max() -> BigUint {
                $max
            }

            #[allow(dead_code)]
            pub fn from_literal(x: u128) -> Self {
                let big_x = BigUint::from(x);
                if big_x > $name::max().into() {
                    panic!("literal too big for type {}", stringify!($name));
                }
                big_x.into()
            }
        }
    };
}

#[macro_export]
macro_rules! define_abstract_integer_modular {
    ($name:ident, $bytes:literal, $max:expr) => {
        define_abstract_integer_struct!($name, $bytes, $max);
        define_abstract_integer_modular_operations!($name, $bytes);
    };
}

#[macro_export]
macro_rules! define_abstract_integer_bounded {
    ($name:ident, $bytes:literal, $max:expr) => {
        define_abstract_integer_struct!($name, $bytes, $max);
        define_abstract_integer_bounded_operations!($name, $bytes);
    };
}

mod tests;
