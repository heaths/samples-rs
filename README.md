# Samples

This repository contains samples of unit tests, integration tests, and examples.

![Samplesaurus](https://heaths.github.io/samples-rs/assets/samplesaurus_240.png)

## Code sample

An example of code in a separate markdown document:

```rust
use samples::say_hello;

let greeting = say_hello("world");
assert_eq!("hello, world!", &greeting);
```

## Documentation

You can also test samples in [separate documentation](docs/index.md) in much the same way.
By injecting a single markdown file per module, line numbers

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
