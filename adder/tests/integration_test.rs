use adder;

#[test]
fn external_test() {
    assert_eq!(4, adder::add_two(2));
}
