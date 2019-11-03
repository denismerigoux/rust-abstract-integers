#[allow(unused_imports)]
extern crate uint;
use uint::field;

// TODO: These imports should go somewhere else.
extern crate num;
use num::{BigUint, Num, Zero};
use std::ops::*;

#[test]
fn basic_uint_test() {
    #[field(3fffffffffffffffffffffffffffffffb)]
    struct Poly1305Field;

    let x1 = Poly1305Field::from(24875808327634644);
    let x2 = Poly1305Field::from(91987276365379830);
    let x3 = x1 + x2;
    assert_eq!(Poly1305Field::from(116863084693014474u128), x3.into());

    let x1 = Poly1305Field::from("10498385709182435134");
    let x2 = Poly1305Field::from("9871425437538592414723");
    let x3 = x1 + x2;
    assert_eq!(Poly1305Field::from("98818bd7bcc41714849857"), x3.into());

    let x1 = Poly1305Field::from(24875808327634644);
    let x2 = Poly1305Field::from(91987276365379830);
    let x3 = x1 + x2;
    assert_eq!(Poly1305Field::from(116863084693014474u128), x3.into());

    #[field(FF)]
    struct SmallModular;

    let x1 = SmallModular::from(254);
    let x2 = SmallModular::from(3);
    let x3 = x1 + x2;
    assert_eq!(SmallModular::from(2), x3.into());
    let x4 = SmallModular::from(5);
    let x5 = x3 - x4;
    assert_eq!(SmallModular::from(252), x5.into());
    let x6 = x5 / SmallModular::from(4);
    assert_eq!(SmallModular::from(63), x6.into());
}
