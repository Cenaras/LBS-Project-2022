use proc_macro::{self, TokenStream, Literal};
use quote::{quote, ToTokens};
use syn::fold::{self, Fold};
use syn::{parse_macro_input, parse_quote, Block, DeriveInput, Ident, ItemFn, ItemStruct, LitStr};
use syn::parse::{Parse};

fn insert_print(function_level: LitStr, node: ItemFn) -> ItemFn {
    let block = node.block;
    let mut stmts = block.stmts;
    let required_level = function_level.value();

    stmts.insert(
        0,
        parse_quote!{
            let level = self.get_level(lattice_contracts, caller_address);
        },
    );
    stmts.insert(
        1,
        parse_quote!{
            assert!(self.le(lattice_contracts, &#required_level.to_string(), &level));
        },
    );
    // Return updated node, with inserted stuff in block stamtements
    ItemFn {
        attrs: node.attrs,
        vis: node.vis,
        sig: node.sig,
        block: Box::new(Block {
            brace_token: block.brace_token,
            stmts,
        }),
    }
}

#[proc_macro_attribute]
pub fn level(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let arg = parse_macro_input!(attr as LitStr);

    let output = insert_print(arg, input_fn);
    TokenStream::from(quote!{#output})
}

#[proc_macro_attribute]
pub fn lattice_address(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let name = input_struct.ident.clone();
    let addr = parse_macro_input!(attr as LitStr);
    let addr2 = &stringify!(#addr).to_string();
    TokenStream::from(quote! {
        #input_struct
        impl #name {
            fn raise_level(address: &String, level: &String) {
                // blockchain_call simulates a blockchain transaction to the entity present at its argument
                self.get_lattice_contract(&stringify!(#addr).to_string()).raise_level(&self.address, address, level);
            }

            fn le(level1: &String, level2: &String) {
                self.get_lattice_contract(&stringify!(#addr).to_string()).le(level1, level2);
            }
        }
    })
}
/*
impl State {
    fn raise_level(address: String, level: String) {
        get_address_struct(#lattice_address).raise_level(address, level);
    }

    fn le(level1: String, level2: String) {
        get_address_struct(#lattice_address).le(level1, level2);
    }
}
*/
