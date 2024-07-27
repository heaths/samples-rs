#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://heaths.github.io/samples-rs/assets/samplesaurus_100.png")]

#[cfg(doctest)]
mod docs;
mod subjects;
#[cfg(test)]
mod test;

use std::fmt;
pub use subjects::*;

/// Say hello to whoever you specify.
///
/// # Examples
///
/// ```
/// # use samples::say_hello;
///
/// let greeting = say_hello("world");
/// assert_eq!("hello, world!", &greeting);
/// ```
pub fn say_hello(who: impl fmt::Display) -> String {
    format!("hello, {}!", who.to_string())
}

#[test]
fn test_say_hello_num() {
    let greeting = say_hello(1.to_string());
    assert_eq!("hello, 1!", greeting);
}

/// Say hello to the world.
///
/// # Examples
///
/// ```
/// # use samples::hello_world;
///
/// let greeting = hello_world();
/// assert_eq!("hello, world!", greeting);
/// ```
pub fn hello_world() -> &'static str {
    "hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct World;

    impl fmt::Display for World {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("world")
        }
    }

    #[test]
    fn say_hello_world() {
        let greeting = say_hello(World);
        assert_eq!("hello, world!", &greeting);
    }
}
