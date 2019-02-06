extern crate num;
use num::BigUint;
use std::ops::*;

macro_rules! define_abstract_integer_bounded_operations {
    ($name:ident, $bits:literal) => {
        impl Add for &$name {
            type Output = $name;
            fn add(self, rhs: &$name) -> $name {
                let c = self.0.clone() + rhs.0.clone();
                if c > $name::max().0 {
                    panic!("bounded addition overflow for type {}", stringify!($name));
                }
                $name(c)
            }
        }
    };
}

macro_rules! define_abstract_integer_modular_operations {
    ($name:ident, $bits:literal) => {
        impl Add for &$name {
            type Output = $name;
            fn add(self, rhs: &$name) -> $name {
                let c = (self.0.clone() + rhs.0.clone()) % $name::max().0;
                if c > $name::max().0 {
                    panic!("bounded addition overflow for type {}", stringify!($name));
                }
                $name(c)
            }
        }
    };
}

macro_rules! define_abstract_integer_common_operations {
    ($name:ident, $bits:literal) => {};
}

macro_rules! define_abstract_integer_struct {
    ($name:ident, $bits:literal) => {
        /// Little endian byte representation of the integer
        #[derive(Debug)]
        pub struct $name(BigUint);
    };
}

macro_rules! define_abstract_integer_modular {
    ($name:ident, $bits:literal, $max:expr) => {
        define_abstract_integer_struct!($name, $bits);

        impl $name {
            fn max() -> Self {
                $name($max)
            }

            pub fn from_literal(x: u128) -> Self {
                let big_x = BigUint::from(x);
                if big_x > $name::max().0 {
                    panic!("literal too big for type {}", stringify!($name));
                }
                $name(big_x)
            }
        }

        define_abstract_integer_common_operations!($name, $bits);
        define_abstract_integer_modular_operations!($name, $bits);
    }
}

macro_rules! define_abstract_integer_bounded {
    ($name:ident, $bits:literal, $max:expr) => {
        define_abstract_integer_struct!($name, $bits);

        impl $name {
            fn max() -> Self {
                $name($max)
            }

            pub fn from_literal(x: u128) -> Self {
                let big_x = BigUint::from(x);
                if big_x > $name::max().0 {
                    panic!("literal too big for type {}", stringify!($name));
                }
                $name(big_x)
            }
        }

        define_abstract_integer_common_operations!($name, $bits);
        define_abstract_integer_bounded_operations!($name, $bits);
    };
}

define_abstract_integer_modular!(Felem, 19, BigUint::from(255u32));
define_abstract_integer_bounded!(FelemRepr, 16, BigUint::from(0xFFFFu16));

#[test]
fn wrapping() {
    let x1 = &Felem::from_literal(254);
    let x2 = &Felem::from_literal(3);
    let _x3 = x1 + x2;
}

#[test]
#[should_panic]
fn bounded() {
    let y1 = &FelemRepr::from_literal(65530);
    let y2 = &FelemRepr::from_literal(6);
    let _y3 = y1 + y2;
}
