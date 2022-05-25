use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::fold::{self, Fold};
use syn::{parse_macro_input, parse_quote, Block, DeriveInput, Ident, ItemFn, ItemStruct};

fn insert_print(id: Ident, node: ItemFn) -> ItemFn {
    let block = node.block;
    let mut stmts = block.stmts;
    stmts.insert(
        0,
        parse_quote! {
            println!("Arg is {}", stringify!(#id));
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
    let arg: Ident = syn::parse(attr).unwrap();

    let output = insert_print(arg, input_fn);
    TokenStream::from(quote! {#output})
}

#[proc_macro_attribute]
pub fn lattice_address(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let name = input_struct.ident.clone();
    TokenStream::from(quote! {
        #input_struct
        impl #name {
            fn print() {
                println!("Hello macro");
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
