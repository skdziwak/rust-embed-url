//! # rust-embed-url
//!
//! A procedural macro for embedding URL content at compile time with SHA-256 hash verification.
//!
//! ## Features
//!
//! - Fetches URL content during compilation
//! - Verifies content integrity using SHA-256 checksums
//! - Returns embedded bytes as `Box<[u8]>`
//! - Fails fast with clear error messages on mismatch or network failures
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_embed_url::embed_url;
//!
//! fn main() {
//!     let data = embed_url!(
//!         "https://example.com",
//!         "ZmY2N2E5ZDc2NGQ2YTIzNjdhMTg3NzM0ZTY5N2Y2YTUzMjE3ZGI5YTIxYzEwMWQ0MTBhMTEzY2E4NzFhMjk5ZAo="
//!     );
//!     let content = String::from_utf8(data.into()).unwrap();
//!     println!("{}", content);
//! }
//! ```
//!
//! ## Computing the Hash
//!
//! To compute the SHA-256 hash (base64 encoded) for your URL:
//!
//! ```bash
//! curl -s "https://example.com" | sha256sum | cut -d' ' -f1 | base64
//! ```

use base64::prelude::*;
use reqwest::Url;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use syn::{LitStr, Token, parse::Parse, parse_macro_input};

/// Parsed macro input containing URL and expected hash.
struct UrlWithHash {
    /// The URL to fetch content from.
    url: LitStr,
    /// Expected SHA-256 hash (base64 encoded) of the content.
    hash: LitStr,
}

impl Parse for UrlWithHash {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let url = input.parse()?;
        let _: Token![,] = input.parse()?;
        let hash = input.parse()?;
        Ok(UrlWithHash { url, hash })
    }
}

/// Computes SHA-256 hash of bytes and returns base64-encoded result.
fn hash_bytes_b64_sha256(data: &[u8]) -> String {
    BASE64_STANDARD.encode(Sha256::digest(data))
}

/// Procedural macro that embeds URL content at compile time with hash verification.
///
/// # Arguments
///
/// * `url` - The URL to fetch (string literal)
/// * `hash` - Expected SHA-256 hash of the content, base64 encoded (string literal)
///
/// # Returns
///
/// Returns a `Box<[u8]>` containing the fetched content.
///
/// # Compile-time Errors
///
/// This macro will fail to compile if:
/// - The URL is invalid
/// - The network request fails
/// - The response status is not 2xx
/// - The computed hash doesn't match the expected hash
///
/// # Example
///
/// ```rust,ignore
/// let html = embed_url!("https://example.com", "ZmY2N2E5...");
/// ```
#[proc_macro]
pub fn embed_url(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UrlWithHash { url, hash } = parse_macro_input!(item as UrlWithHash);

    let url_str = url.value();
    let expected_hash = hash.value();

    // Validate and parse URL
    let url = Url::parse(url_str.as_str()).unwrap_or_else(|e| {
        panic!("Invalid URL '{}': {}", url_str, e);
    });

    // Fetch content from URL
    let response = reqwest::blocking::get(url.as_str()).unwrap_or_else(|e| {
        panic!("Failed to fetch URL '{}': {}", url_str, e);
    });

    // Verify HTTP status code
    let status_code = response.status().as_u16();
    if !(200..300).contains(&status_code) {
        let error_body = response.text().unwrap_or_default();
        panic!(
            "Failed to fetch URL '{}': HTTP {} {}",
            url_str,
            status_code,
            error_body.lines().next().unwrap_or("Unknown error")
        );
    }

    // Extract response bytes
    let bytes = response.bytes().unwrap_or_else(|e| {
        panic!("Failed to read response body from '{}': {}", url_str, e);
    });

    // Verify content integrity using SHA-256 hash
    let computed_hash = hash_bytes_b64_sha256(&bytes);
    if computed_hash != expected_hash {
        panic!(
            "Hash verification failed for URL '{}'.\n\
             Expected: {}\n\
             Computed: {}",
            url_str, expected_hash, computed_hash
        );
    }

    // Generate Rust byte array literal
    let byte_literals: Vec<String> = bytes.iter().map(|b| format!("0x{:02x}", b)).collect();
    let array_content = byte_literals.join(",");

    // Return as Box<[u8]> via intermediate array
    let output = format!(
        "{{ let data = [{}]; let bytes: Box<[u8]> = data.into(); bytes }}",
        array_content
    );

    proc_macro::TokenStream::from_str(&output).unwrap()
}
