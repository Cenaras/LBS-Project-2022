extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, parse_macro_input, DeriveInput};

struct Args{
    vars:HashSet<Ident>
}

impl Parse for Args{
    fn parse(input: ParseStream) -> Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);
    let met = parse_macro_input!(_metadata as Args);
    let name = input.ident;
    let attrs = input.attrs;
    println!("Name is {}", name);
    println!("Name is {:?}", met.vars);
    TokenStream::from(quote!{struct H{}})
}











#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
