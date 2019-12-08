extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use syn::parse::{self, Parse, ParseStream};

#[derive(Debug)]
struct Assertified(syn::Expr);

impl Parse for Assertified {
    fn parse(input: ParseStream) -> parse::Result<Assertified> {
        let expr = input.parse()?;
        Ok(Assertified(expr))
    }
}

impl ToTokens for Assertified {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            syn::Expr::Binary(expr) => {
                // FIXME? ignore attributes
                // FIXME? b"ab" comes out as [97, 98]
                let actual = &expr.left;
                let op = &expr.op;
                let expected = &expr.right;
                tokens.extend(quote!({
                    let actual = #actual;
                    let expected = #expected;
                    if !(actual #op expected) {
                        panic!(
                            "failed: {}\n  \
                              actual:   {:?}\n  \
                              expected: {:?}\n",
                            stringify!(#expr), actual, expected);
                    }
                }));
            }
            _ => {
                panic!("what am I supposed to to do??? {:?}", self);
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

#[proc_macro_hack]
pub fn assertify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let assertified = parse_macro_input!(input as Assertified);
    quote!(#assertified).into()
}

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
