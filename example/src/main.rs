//! Example demonstrating rust-embed-url macro usage.
//!
//! This example fetches and embeds the content of https://example.com
//! at compile time with SHA-256 hash verification.

use rust_embed_url::embed_url;

fn main() {
    // Embed the HTML content from example.com at compile time
    let data = embed_url!(
        "https://example.com",
        "/2ep12TWojZ6GHc05pf2pTIX25ohwQHUEKETyocaKZ0="
    );

    // Convert bytes to string and display
    let content = String::from_utf8(data.into()).expect("Failed to parse UTF-8");

    println!("Embedded content from example.com:");
    println!("{}", content);
}
