# Change log

All notable changes to this project will be documented in this file.

## main branch

### Changes

* Consolidated old assertify_proc_macros crate into this crate. This crate only
  defines the procedural macros and does not actually use them, so there is no
  need for a sub-crate.
* Removed use of [proc_macro_hack][], which hasn’t been needed since Rust 1.45
  (released in July 2020).
* Added this change log.

[proc_macro_hack]: https://docs.rs/proc-macro-hack/0.5.19/proc_macro_hack/
