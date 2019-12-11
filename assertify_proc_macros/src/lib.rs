extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use syn::parse::{self, Parse, ParseStream};

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
                syn::BinOp::Eq(_) | syn::BinOp::Ne(_)
                    | syn::BinOp::Lt(_) | syn::BinOp::Le(_)
                    | syn::BinOp::Gt(_) | syn::BinOp::Ge(_)
                => {
                    return Ok(Assertified::BinaryExpr(expr.clone()));
                }
                _ => {},
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
                        panic!("failed: {}\n", stringify!(#expr));
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

// Convert an expression into a call to assert
//
// Docs are in the wrapper crate (assertify).
#[proc_macro_hack]
pub fn assertify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let assertified = parse_macro_input!(input as Assertified);
    quote!(#assertified).into()
}

// Convert an expression into a test function
//
// Docs are in the wrapper crate (assertify).
#[proc_macro]
pub fn testify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Testified{name, assertion} = parse_macro_input!(input as Testified);
    quote!(
        #[test]
        fn #name() {
            #assertion;
        }
    ).into()
}
