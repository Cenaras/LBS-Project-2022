use proc_macro::{self, TokenStream};
use quote::quote;
use syn::fold::{self, Fold};
use syn::parse::Parser;
use syn::Fields::Named;
use syn::{parse_macro_input, parse_quote, Expr, Field, Ident, ItemFn, ItemStruct, LitStr};

fn insert_arguments(node: &mut ItemFn) {
    node.sig.inputs.insert(
        1,
        parse_quote! {lattice_contracts: &mut HashMap<String, MyLatticeContract>},
    );
    node.sig
        .inputs
        .insert(2, parse_quote! {caller_address: &String});
}

fn insert_assertion(function_level: LitStr, node: &mut ItemFn) {
    let required_level = function_level.value();

    node.block.stmts.insert(
        0,
        parse_quote! {let level = self.get_level(caller_address);},
    );
    node.block.stmts.insert(
        1,
        parse_quote! {let is_allowed = self.flows_to(&#required_level.to_string(), &level);},
    );
    node.block
        .stmts
        .insert(2, parse_quote! {assert!(is_allowed);});
}

struct InsertMethodCallArg;

impl Fold for InsertMethodCallArg {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let mut folded_e = fold::fold_expr(self, expr);
        match &mut folded_e {
            Expr::MethodCall(expr_method_call) => {
                let ident = expr_method_call.method.clone();
                let raise_level: Ident = parse_quote! {raise_level};
                let flows_to: Ident = parse_quote! {flows_to};
                let get_level: Ident = parse_quote! {get_level};
                if ident == raise_level || ident == flows_to || ident == get_level {
                    expr_method_call
                        .args
                        .insert(0, parse_quote! {lattice_contracts});
                    Expr::MethodCall(expr_method_call.to_owned())
                } else {
                    folded_e
                }
            }
            _ => folded_e,
        }
    }
}

#[proc_macro_attribute]
pub fn level(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let arg = parse_macro_input!(attr as LitStr);

    insert_arguments(&mut input_fn);
    insert_assertion(arg, &mut input_fn);
    let output = InsertMethodCallArg.fold_item_fn(input_fn);
    TokenStream::from(quote! {#output})
}

fn insert_address_field(node: &mut ItemStruct) {
    match &mut node.fields {
        Named(fields_named) => fields_named.named.insert(
            0,
            Field::parse_named
                .parse2(quote! { pub address: String })
                .unwrap(),
        ),
        _ => {}
    };
}

#[proc_macro_attribute]
pub fn lattice_address(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_struct = parse_macro_input!(item as ItemStruct);
    insert_address_field(&mut input_struct);
    let name = input_struct.ident.clone();
    let addr = parse_macro_input!(attr as LitStr).value();
    TokenStream::from(quote! {
        #input_struct
        impl #name {
            fn raise_level(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, address: &String, level: &String) {
                // lattice_contracts.get() simulates a blockchain transaction to the entity present at its argument, i.e. the given lattice contract
                lattice_contracts.get_mut(&#addr.to_string()).unwrap().raise_level(&self.address, address, level);
            }

            fn flows_to(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, level1: &String, level2: &String) -> bool {
                lattice_contracts.get_mut(&#addr.to_string()).unwrap().flows_to(level1, level2)
            }

            fn get_level(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, address: &String) -> String {
                lattice_contracts.get_mut(&#addr.to_string()).unwrap().get_level(address)
            }
        }
    })
}
