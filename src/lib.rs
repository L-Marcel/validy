extern crate self as validy;
#[cfg(feature = "axum")]
pub mod axum;
pub mod builders;
#[doc = include_str!("../readme.md")]
pub mod core;
pub mod functions;
mod impls;
pub mod settings;
pub mod utils;
