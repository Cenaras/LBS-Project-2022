
use proc_macro::{self, TokenStream};
use syn::{DeriveInput, Ident};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn my_custom_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let arg: Ident = syn::parse(attr).unwrap();
    let name: Ident = ast.ident.clone();
    //ast.ident = arg;
    println!("AST of item is: {}", name);

    
    impl_hello_macro(&ast)
    //ast.to_token_stream()
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
