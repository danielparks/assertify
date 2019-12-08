# Assertify

Convenience macros to make Rust code tests from simple expressions.

```rust
testify!(concat_literals, concat("a", "b") == "ab");

#[test]
fn my_test() {
  assertify!(concat("a", "b") == "ab");
}
```

## To do

  * Test failures
  * Test compilation failures
  * Fix compilation failure errors
  * Improve failure messages when for ops other than ==
  * Document macros

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.
