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

/// Argument macro which wraps the SQLx `sqlx::test` macro to bootstrap and setup our backend services
/// for testing, and allows for safe shutdown of the Actix server.
///
/// Note: Only uses the `PgPoolOptions, PgConnectOptions` arguments.
///
/// Note: Any drop implementation of the server/application should not include handling the shutdown
/// of the server. This is handled by the macro so that if there is a panic, it can still be shutdown
/// correctly, and the panic can continue.
///
/// Example setup function:
///
/// ```no_run
/// async fn setup_service(
/// 	fixtures: &[Fixture],
/// 	services: &[Service],
/// 	pool_opts: PgPoolOptions,
/// 	conn_opts: PgConnectOptions
/// ) -> (ServerHandle, u16) {
///     let app = initialize_server(fixtures, services, pool_opts, conn_opts).await;
///
///     let handle = app.handle();
///     let port = app.port();
///
///     let _join_handle = tokio::spawn(app.run_until_stopped());
///
///     (handle, port)
/// }
/// ```
///
/// The macro takes three arguments:
///
/// - `setup` **Required**: The setup function, for example `setup = "my_service_setup"`;
/// - `fixtures` **Optional**: A list of `T` - Passed as the `fixtures` argument to the setup fn;
/// - `services` **Optional**: A list of `T` - Passed as the `services` argument to the setup fn.
///
/// ```no_run
/// #[test_service(
/// 	setup = "setup_service",
/// 	fixtures("Fixture::User", "Fixture::Jig"),
/// 	services("Service::S3", "Service::Email"),
/// )
/// ```
///
/// Example test case:
///
/// ```no_run
/// use macros::test_service;
/// use crate::{fixture::Fixture, service::Service};
///
/// //...
///
/// async fn setup_service(
///     fixtures: &[Fixture],
///     services: &[Service],
///     pool_opts: PgPoolOptions,
///     conn_opts: PgConnectOptions
/// ) -> (ServerHandle, u16) {
///     // ... do setup
///
///     (handle, port)
/// }
///
/// #[test_service(setup = "setup_service", fixtures("Fixture::User"))]
/// async fn create_default(
///     port: u16, // <-- NB
/// ) -> anyhow::Result<()> {
///     let settings = insta::Settings::clone_current();
///
///     let client = reqwest::Client::new();
///
///     let resp = client
///         .post(&format!("http://0.0.0.0:{}/v1/resource", port))
///         .login()
///         .send()
///         .await?
///         .error_for_status()?;
///
///     assert_eq!(resp.status(), StatusCode::CREATED);
///
///     let body: CreateResponse<ResourceId> = resp.json().await?;
///
///     settings
///         .bind_async(async {
///             assert_json_snapshot!(body, {".id" => "[id]"});
///         })
///         .await;
///
///     let resource_id = body.id.0;
///
///     let resp = client
///         .get(&format!(
///             "http://0.0.0.0:{}/v1/resource/{}/draft",
///             port, resource_id
///         ))
///         .login()
///         .send()
///         .await?
///         .error_for_status()?;
///
///     let body: serde_json::Value = resp.json().await?;
///
///     insta::assert_json_snapshot!(
///         body, {
///             ".**.id" => "[id]",
///             ".**.createdAt" => "[created_at]",
///             ".**.lastEdited" => "[last_edited]"});
///
///     Ok(())
/// }
/// ```
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

                    use ::futures::FutureExt;
                    use ::std::panic::AssertUnwindSafe;
                    let result = AssertUnwindSafe(wrapped(port)).catch_unwind().await;

                    server_handle.stop(true).await;

                    match result {
                        Ok(result) => result,
                        Err(error) => ::std::panic::resume_unwind(error),
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
