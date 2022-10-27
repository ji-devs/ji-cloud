extern crate proc_macro;

use make_path_parts_proc::PathPartsInput;
use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DeriveInput, ExprArray, Lit, MetaList, NestedMeta, Result};

mod make_path_parts_proc;
mod path_part_derive;

use quote::{format_ident, quote};

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

fn parse_list(list: &MetaList) -> Result<ExprArray> {
    let mut items = vec![];
    for nested in &list.nested {
        match nested {
            syn::NestedMeta::Lit(syn::Lit::Str(value)) => {
                items.push(value.value());
            }
            other => {
                return Err(syn::Error::new_spanned(other, "expected string literal"));
            }
        }
    }

    let mut concatenated = items
        .into_iter()
        .reduce(|acc, value| acc + "," + &value)
        .unwrap_or_default();
    concatenated = format!("[{concatenated}]");

    let concatenated = match concatenated.parse::<TokenStream>() {
        Err(error) => {
            return Err(syn::Error::new_spanned(list, error));
        }
        Ok(concatenated) => concatenated,
    };

    Ok(syn::parse::<syn::ExprArray>(concatenated)?)
}

type SetupFn = syn::Ident;
type Fixtures = ExprArray;
type Services = ExprArray;

fn parse_args(args: Vec<NestedMeta>) -> Result<(SetupFn, Fixtures, Services)> {
    let mut setup_fn = None;
    let mut fixtures = None;
    let mut services = None;

    for arg in args.iter() {
        match arg {
            NestedMeta::Meta(syn::Meta::List(list)) => {
                if list.path.is_ident("fixtures") {
                    fixtures = Some(parse_list(list)?);
                } else if list.path.is_ident("services") {
                    services = Some(parse_list(list)?);
                }
            }
            NestedMeta::Meta(syn::Meta::NameValue(namevalue)) => {
                if namevalue.path.is_ident("setup") {
                    if let Lit::Str(value) = &namevalue.lit {
                        setup_fn = Some(format_ident!("{}", value.value()));
                    } else {
                        return Err(syn::Error::new_spanned(
                            namevalue,
                            "expected string literal",
                        ));
                    }
                }
            }
            other => {
                return Err(syn::Error::new_spanned(other, "unexpected argument"));
            }
        }
    }

    let setup_fn = match setup_fn {
        None => {
            let args = args.iter();
            return Err(syn::Error::new_spanned(quote!({#(#args)*}), "foo"));
        }
        Some(setup_fn) => setup_fn,
    };

    let default_expr_array =
        || syn::parse::<syn::ExprArray>("[]".parse::<TokenStream>().unwrap()).unwrap();

    Ok((
        setup_fn,
        fixtures.unwrap_or_else(default_expr_array),
        services.unwrap_or_else(default_expr_array),
    ))
}

#[proc_macro_attribute]
pub fn test_service(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    let input = syn::parse_macro_input!(input as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;

    match parse_args(args) {
        Ok((setup_fn, fixtures, services)) => {
            quote! {
                #[sqlx::test]
                pub async fn #name(
                    pool_opts: PgPoolOptions,
                    conn_opts: PgConnectOptions,
                ) #ret {
                    async fn wrapped(port: u16) #ret {
                        #body
                    }

                    let (server_handle, port) = #setup_fn(&#fixtures, &#services, pool_opts, conn_opts).await;

                    use futures::FutureExt;
                    use std::panic::AssertUnwindSafe;
                    let result = AssertUnwindSafe(wrapped(port)).catch_unwind().await;

                    server_handle.stop(true).await;

                    match result {
                        Ok(result) => result,
                        Err(error) => std::panic::resume_unwind(error),
                    }
                }
            }
            .into()
        }
        Err(error) => {
            let error = error.to_compile_error();
            return quote!(#error).into();
        }
    }
}
