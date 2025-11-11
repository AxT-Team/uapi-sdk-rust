//! UAPI Rust SDK
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
pub mod errors;
pub mod models;
pub mod services;

pub use client::{Client, ClientBuilder};
pub type Result<T> = std::result::Result<T, errors::Error>;

#[doc(hidden)]
pub use reqwest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_exists() {
        let _ = Client::builder();
    }
}
