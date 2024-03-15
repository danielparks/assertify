# Deprecated — use [assert2][]

Use [assert2][] instead of this crate. `assertify!` can be replaced by the
more capable [`assert2::assert!`][] everywhere, and `testify!` can implemented
with a short macro:

```rust
macro_rules! testify {
    ($name:ident, $($test:tt)+) => {
        #[test]
        fn $name() {
            ::assert2::assert!($($test)+);
        }
    };
}
```

[assert2]: https://crates.io/crates/assert2
[`assert2::assert!`]: https://docs.rs/assert2/0.3.7/assert2/macro.assert.html
