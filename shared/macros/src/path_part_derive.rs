use proc_macro::TokenStream;
use quote::quote;

pub fn impl_path_part(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    TokenStream::from(quote! {
        impl PathPart for #name {
            fn get_path_string(&self) -> String {
                self.0.to_string()
            }
        }
    })
}
