//! This provides two convenience macros to make tests from simple expressions.
//! The failure messages produced by these macros are more instantly obvious
//! than those of `assert_eq!` and friends, which often require looking at the
//! source code to see the expression that failed.
//!
//! See [assertify!](macro.assertify.html) for an example of a failure message.
//!
//! ```rust
//! # #[macro_use] extern crate assertify;
//! # fn main() {}
//! testify!(add_one_two, 1 + 2 == 3);
//! ```
//!
//! ```rust
//! #[test]
//! fn add_one_two() {
//!     assertify!(1 + 2 == 3);
//! }
//! ```

use proc_macro_hack::proc_macro_hack;

/// Assert an expression is true or give a useful error when it isn’t.
///
/// If the expression contains a comparison, e.g. `==`, then the failure message
/// will display the value of both sides of the comparison. Note that the
/// _right_ side will be listed as the “expected” value — think “right” as in
/// “correct.”
///
/// # Examples
///
/// ## Error for a failed comparison
///
/// ```should_panic
/// # #[macro_use] extern crate assertify;
/// # fn main() {
/// assertify!(1 + 2 == 0);
/// # }
/// ```
///
/// Produces:
///
/// ```text
/// ---- tests::simple_eq stdout ----
/// thread 'tests::simple_eq' panicked at 'failed: 1 + 2 == 0
///   actual:      3
///   expected: == 0
/// ', src/lib.rs:96:9
/// ```
///
/// ## Error for other failures
///
/// ```should_panic
/// # #[macro_use] extern crate assertify;
/// # fn main() {
/// assertify!(false);
/// # }
/// ```
///
/// Produces:
///
/// ```text
/// ---- tests::simple_literal stdout ----
/// thread 'tests::simple_literal' panicked at 'failed: false', src/lib.rs:131:9
/// ```
#[proc_macro_hack]
pub use assertify_proc_macros::assertify;

/// Create a test function from an expression.
///
/// `testify!` is essentially a wrapper around [`assertify!`]. It takes two
/// arguments:
///
/// 1. `name`: A name for the test (as a bareword — don’t use quotes).
/// 2. `expression`: The expression to be tested with [`assertify!`].
///
/// # Examples
///
/// The following two examples are equivalent:
///
/// ```rust
/// # #[macro_use] extern crate assertify;
/// # fn main() {}
/// testify!(add_one_two, 1 + 2 == 3);
/// ```
///
/// ```rust
/// #[test]
/// fn add_one_two() {
///     assertify!(1 + 2 == 3);
/// }
/// ```
///
/// [`assertify!`]: macro.assertify.html
pub use assertify_proc_macros::testify;

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn trybuild_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild-failures/*.rs");
    }

    #[test]
    fn assertify_simple_expr() {
        assertify!(1 - 2 == -1);
    }

    testify!(simple_eq, 1 + 2 == 3);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    testify!(add_pos, add(1, 2) == 3);
    testify!(add_neg, add(-1, 2) == 1);
    testify!(add_all_expressions, add(add(1, 1), 5 - 3) == 2 + 5 - 3);

    fn concat(a: &str, b: &str) -> String {
        let mut s = String::with_capacity(a.len() + b.len());
        s.push_str(a);
        s.push_str(b);
        s
    }

    testify!(concat_literal, concat("a", "b") == "ab");

    fn concat_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(a.len() + b.len());
        v.extend_from_slice(a);
        v.extend_from_slice(b);
        v
    }

    testify!(concat_bytes_literals, concat_bytes(b"a", b"b") == b"ab");

    fn result(good: bool) -> Result<(), &'static str> {
        if good {
            Ok(())
        } else {
            Err("bad")
        }
    }

    testify!(literal_true, true);
    testify!(boolean_logic, true && true);

    testify!(result_ok, result(true) == Ok(()));
    testify!(result_unwrap, result(true).unwrap() == ());
    testify!(result_err, result(false) == Err("bad"));
    testify!(result_not_ok, result(false) != Ok(()));
    testify!(result_not_err, result(false) != Err("nope"));

    #[test]
    #[should_panic(
        expected = "failed: 1 + 2 == 0\n  actual:      3\n  expected: == 0\n"
    )]
    fn fail_simple_eq() {
        assertify!(1 + 2 == 0);
    }

    #[test]
    #[should_panic(expected = "failed: false")]
    fn fail_simple_literal() {
        assertify!(false);
    }

    #[test]
    #[should_panic(
        expected = "failed: 1 + 2 != 3\n  actual:      3\n  expected: != 3\n"
    )]
    fn fail_simple_ne() {
        assertify!(1 + 2 != 3);
    }

    #[test]
    #[should_panic(
        expected = "failed: 1 + 2 > 4\n  actual:     3\n  expected: > 4\n"
    )]
    fn fail_simple_gt() {
        assertify!(1 + 2 > 4);
    }

    #[test]
    #[should_panic(
        expected = "failed: result(false) == Ok(())\n  actual:      Err(\"bad\")\n  expected: == Ok(())\n"
    )]
    fn fail_result_ok() {
        assertify!(result(false) == Ok(()));
    }
}
