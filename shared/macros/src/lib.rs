extern crate proc_macro;

use make_path_parts_proc::PathPartsInput;
use proc_macro::TokenStream;
use syn;
use syn::{parse_macro_input, DeriveInput};

mod make_path_parts_proc;
mod path_part_derive;

#[proc_macro]
pub fn make_path_parts(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let parts = PathPartsInput::try_from(input).unwrap();

    // let output: proc_macro2::TokenStream = parts.into();
    let output: proc_macro2::TokenStream = parts.try_into().unwrap();

    output.into()
}

/// Derive macro that to implements PathPart for IDs.
/// Only works on tuple structs with a single value value that implements
/// ToString like Uuid or u32.
/// If you wanna implement PathPart on types that don'w follow this exact
/// structure do it manually
#[proc_macro_derive(PathPart, attributes(my_trait))]
pub fn path_part(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    path_part_derive::impl_path_part(&ast)
}
