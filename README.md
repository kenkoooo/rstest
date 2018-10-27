# A simple `pytest` clone for Rust

`rstest` use procedural macro to implement simple fixtures and table
based tests. To use it you need at least 1.30 toolchain and add follow
lines to your `Cargo.toml` file:

```
[dev-dependencies]
rstest = "0.2"
```

The core idea is that every input arguments of your test function will
be resolved by call a function with the same name. You can also use
`mut` argument or generic types.

Example:

```rust
#[cfg(test)]
extern crate rstest;

use rstest::rstest;

pub fn fixture() -> u32 { 42 }

#[rstest]
fn should_success(fixture: u32) {
    assert_eq!(fixture, 42);
}

#[rstest]
fn should_fail(fixture: u32) {
    assert_ne!(fixture, 42);
}
```

Moreover you can use `rstest_parametrize` attribute to implement table
based tests.  An example is the best way to explain it

```rust
#[cfg(test)]
extern crate rstest;

use rstest::rstest_parametrize;

#[rstest_parametrize(
    expected, input,
    case(4, "ciao"),
    case(3, "Foo")
)]
fn strlen_test(expected: usize, input: &str) {
    assert_eq!(expected, input.len());
}
```

You can find more examples in [`resources`](resources) directory and in [`rs8080`](https://github.com/la10736/rs8080/blob/master/src/cpu/test.rs) that use this module intensely.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

