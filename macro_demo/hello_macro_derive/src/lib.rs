use proc_macro::{self, TokenStream};
use syn::{DeriveInput, Ident, parse_macro_input, ItemFn, Block, parse_quote};
use quote::{quote, ToTokens};
use syn::fold::{self, Fold};


/*#[proc_macro_attribute]
pub fn my_custom_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let arg: Ident = syn::parse(attr).unwrap();
    let name: Ident = ast.ident.clone();
    //ast.ident = arg;
    println!("AST of item is: {}", name);
    TokenStream::from(quote!{struct H{}})
    
    //impl_hello_macro(&ast)
    //ast.to_token_stream()
}*/


fn insert_print(id: Ident, node: ItemFn) -> ItemFn {
    let block = node.block;
    let mut stmts = block.stmts;
    stmts.insert(0, parse_quote!{
        println!("Arg is {}", stringify!(#id));
    });
    
    // Return updated node, with inserted stuff in block stamtements
    ItemFn {
        attrs: node.attrs,
        vis: node.vis,
        sig: node.sig,
        block: Box::new(
            Block {
                brace_token: block.brace_token,
                stmts: stmts,
            }
        )
    }
}

#[proc_macro_attribute]
pub fn trace_vars(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let arg: Ident = syn::parse(attr).unwrap();


    let output = insert_print(arg, input_fn);
    TokenStream::from(quote!{#output})
}


/*fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}*/




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
