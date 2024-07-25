#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://heaths.github.io/samples-rs/assets/samplesaurus_100.png")]

#[cfg(any(test, doctest))]
mod docs;

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
pub fn say_hello(who: impl AsRef<str>) -> String {
    format!("hello, {}!", who.as_ref())
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
    struct World {}

    impl AsRef<str> for World {
        fn as_ref(&self) -> &str {
            "world"
        }
    }

    #[test]
    fn say_hello_world() {
        let greeting = say_hello(World {});
        assert_eq!("hello, world!", &greeting);
    }
}
