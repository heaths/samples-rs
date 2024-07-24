use samples::say_hello;

pub fn hello_samplesaurus() -> String {
    say_hello("samplesaurus")
}

#[test]
fn test_hello_samplesaurus() {
    assert_eq!("hello, samplesaurus!", &hello_samplesaurus());
}
