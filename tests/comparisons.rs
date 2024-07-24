mod common;

use samples::{hello_world, say_hello};

#[test]
fn greets_world() {
    assert_eq!(hello_world(), &say_hello(common::WORLD));
}
