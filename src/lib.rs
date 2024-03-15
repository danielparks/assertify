//! **Deprecated. Use [assert2](https://crates.io/crates/assert2) instead.**
//!
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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};
use syn::parse_macro_input;

// FIXME not sure if we care about large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum Assertified {
    BinaryExpr(syn::ExprBinary),
    Other(syn::Expr),
}

impl Parse for Assertified {
    fn parse(input: ParseStream) -> parse::Result<Assertified> {
        let parsed = input.parse()?;
        if let syn::Expr::Binary(expr) = &parsed {
            match expr.op {
                syn::BinOp::Eq(_)
                | syn::BinOp::Ne(_)
                | syn::BinOp::Lt(_)
                | syn::BinOp::Le(_)
                | syn::BinOp::Gt(_)
                | syn::BinOp::Ge(_) => {
                    return Ok(Assertified::BinaryExpr(expr.clone()));
                }
                _ => {}
            }
        }

        Ok(Assertified::Other(parsed))
    }
}

impl ToTokens for Assertified {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Assertified::BinaryExpr(expr) => {
                // FIXME? ignore attributes
                // FIXME? b"ab" comes out as [97, 98]
                let actual = &expr.left;
                let expected = &expr.right;
                let op = &expr.op;
                tokens.extend(quote!({
                    let actual = #actual;
                    let expected = #expected;
                    let op = stringify!(#op);
                    if !(actual #op expected) {
                        panic!(
                            "failed: {expr}\n  \
                              actual:   {sp:width$} {actual:?}\n  \
                              expected: {op:width$} {expected:?}\n",
                            expr=stringify!(#expr),
                            sp="",
                            op=op,
                            width=op.len(),
                            actual=actual,
                            expected=expected);
                    }
                }));
            }
            Assertified::Other(expr) => {
                tokens.extend(quote!({
                    let result: bool = #expr;
                    if !result {
                        panic!("failed: {}", stringify!(#expr));
                    }
                }));
            }
        }
    }
}

#[derive(Debug)]
struct Testified {
    name: syn::Ident,
    assertion: Assertified,
}

impl Parse for Testified {
    fn parse(input: ParseStream) -> parse::Result<Testified> {
        let name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let assertion = input.parse()?;
        Ok(Testified { name, assertion })
    }
}

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
#[deprecated(since = "0.7.0", note = "use assert2::assert! instead")]
#[proc_macro]
pub fn assertify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let assertified = parse_macro_input!(input as Assertified);
    quote!(#assertified).into()
}

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
#[proc_macro]
pub fn testify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Testified { name, assertion } = parse_macro_input!(input as Testified);
    quote!(
        #[test]
        fn #name() {
            #assertion;
        }
    )
    .into()
}
