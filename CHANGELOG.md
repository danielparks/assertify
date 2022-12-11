# Change log

All notable changes to this project will be documented in this file.

## Release 0.7.0 (2022-12-11)

### Changes

* Decided to deprecate. The [assert2][] crate’s macros are strictly better than
  this crate’s `assertify!`, and the `testify!` macro is easy to replace.
* Consolidated old assertify_proc_macros crate into this crate. This crate only
  defines the procedural macros and does not actually use them, so there is no
  need for a sub-crate.
* Removed use of [proc_macro_hack][], which hasn’t been needed since Rust 1.45
  (released in July 2020).
* Added this change log.

[proc_macro_hack]: https://docs.rs/proc-macro-hack/0.5.19/proc_macro_hack/
[assert2]: https://docs.rs/assert2/0.3.7/assert2/
