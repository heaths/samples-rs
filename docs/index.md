# Documentation

Using a similar approach as the project README, you can use integration tests
to test arbitrary documentation.

```rust
let greeting = samples::say_hello("samplesaurus");
assert_eq!("hello, samplesaurus!", &greeting);
```

You can also use different [attributes](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes).

## should_panic

```rust should_panic
let greeting = samples::say_hello("world");
assert_eq!("hello, samplesaurus!", &greeting);
```

## compile_fail

```rust compile_fail
let greeting = samples::say_hello(());
assert_eq!("hello, samplesaurus!", &greeting);
```
