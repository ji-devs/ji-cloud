use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;

pub fn impl_path_part(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let display_name = ast.ident.to_string().to_case(Case::Snake);
    let placeholder = format!("{{{display_name}}}");

    TokenStream::from(quote! {
        impl PathPart for #name {
            const PLACEHOLDER: &'static str = #placeholder;

            fn get_path_string(&self) -> String {
                self.0.to_string()
            }
        }
    })
}
