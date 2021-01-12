#![warn(
    clippy::pedantic,
    clippy::multiple_crate_versions,
    clippy::cognitive_complexity,
    clippy::future_not_send,
    clippy::missing_const_for_fn,
    clippy::needless_borrow,
    clippy::redundant_pub_crate,
    clippy::string_lit_as_bytes,
    clippy::use_self,
    clippy::useless_let_if_seq,
    rust_2018_idioms,
    future_incompatible
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::match_bool,
    clippy::future_not_send,
    clippy::default_trait_access,
    clippy::map_err_ignore,
    // sqlx uses `_expr` a lot
    clippy::used_underscore_binding,
    // sqlx
    clippy::similar_names,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::option_option,
    // come back to this one later
    clippy::too_many_arguments,
)]

pub mod algolia;
pub mod db;
mod domain;
mod error;
mod extractor;
pub mod http;
mod image_ops;
pub mod image_search;
pub mod jwkkeys;
mod jwt;
pub mod logger;
pub(crate) mod more_futures;
pub mod s3;
