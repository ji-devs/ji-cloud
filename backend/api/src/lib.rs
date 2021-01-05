#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]
#![warn(clippy::pedantic)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::future_not_send)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::use_self)]
#![warn(clippy::useless_let_if_seq)]

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

