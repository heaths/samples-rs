# Samples

This [repository](https://github.com/heaths/samples-rs) contains samples of unit tests, integration tests, and examples.

![Samplesaurus](https://heaths.github.io/samples-rs/assets/samplesaurus_240.png)

## Code sample

An example of code in a separate markdown document:

```rust
use samples::say_hello;

let greeting = say_hello("world");
assert_eq!("hello, world!", &greeting);
```

## Documentation

You can also test samples in [separate documentation](https://github.com/heaths/samples-rs/blob/main/docs/index.md) in much the same way.
By injecting a single markdown file per module, line numbers will accurately reference lines of code in the injected markdown file.

You can exclude these modules from release code by attributing them as `#[cfg(doctest)]`.

Note currently that you cannot declare types in code to reference in documentation tests with this predicate. See [rust-lang/rust#67295](https://github.com/rust-lang/rust/issues/67295) for more information and status.

## Testing

To run all unit, integration, and doc tests:

```bash
cargo test
```

To run any tests in examples:

```bash
cargo test --examples
```

To run an example:

```bash
cargo run --example hello
cargo run --example hello -- world
```

## More information

* [Package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
* [`cargo test` command](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
* [`rustdoc` book](https://doc.rust-lang.org/rustdoc/)
* [`rustdoc` documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
