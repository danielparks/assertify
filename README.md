# Assertify and Testify

This provides two convenience macros to make tests from simple Rust expressions.

### `testify!(name, expr)`

Generates a test function named `name` that asserts that `expr` is true.

```rust
testify!(concat_literals, concat("a", "b") == "ab");
```

### `assertify!(expr)`

Generates an assertion for `expr` with a friendly failure message.

```rust
#[test]
fn simple_eq() {
    assertify!(1 + 2 == 0);
}
```

```
---- tests::fail_simple_eq stdout ----
thread 'tests::simple_eq' panicked at 'failed: 1 + 2 == 0
  actual:      3
  expected: == 0
', src/lib.rs:96:9
```

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.
