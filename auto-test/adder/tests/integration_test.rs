use adder::for_integration; // adder should be the lib project name!!!

mod common;

#[test]
fn it_add_two_integration() {
    common::setup();
    assert_eq!(4, for_integration::add_two(2));
}
