extern crate uint;
use uint::field;

#[test]
fn name() {
    #[field(3fffffffffffffffffffffffffffffffb)]
    struct Poly1305Field;

    // let tmp = Poly1305Field::new();
    // println!("field: {:?}", tmp);

    let x1 = Poly1305Field::from_literal(24875808327634644);
    let x2 = Poly1305Field::from_literal(91987276365379830);
    let x3 = x1 + x2;
    assert_eq!(
        Poly1305Field::from_literal(116863084693014474u128),
        x3.into()
    );

    let x1 = Poly1305Field::from_hex("10498385709182435134");
    let x2 = Poly1305Field::from_hex("9871425437538592414723");
    let x3 = x1 + x2;
    assert_eq!(Poly1305Field::from_hex("98818bd7bcc41714849857"), x3.into());

    let x1 = Poly1305Field::from_literal(24875808327634644);
    let x2 = Poly1305Field::from_literal(91987276365379830);
    let x3 = x1 + x2;
    assert_eq!(
        Poly1305Field::from_literal(116863084693014474u128),
        x3.into()
    );
}
