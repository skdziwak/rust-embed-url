# rust-embed-url

A Rust procedural macro for embedding URL content at compile time with SHA-256 hash verification.

## Features

- **Compile-time fetching**: Download URL content during compilation
- **Integrity verification**: SHA-256 checksum validation ensures content hasn't changed
- **Type-safe**: Returns `Box<[u8]>` for easy integration
- **Clear errors**: Descriptive compile-time error messages on failure

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-embed-url = "0.1.0"
```

## Usage

### Basic Example

```rust
use rust_embed_url::embed_url;

fn main() {
    // Embed URL content at compile time with hash verification
    let data = embed_url!(
        "https://example.com", 
        "/2ep12TWojZ6GHc05pf2pTIX25ohwQHUEKETyocaKZ0="
    );
    
    // Use the embedded bytes
    let content = String::from_utf8(data.into()).unwrap();
    println!("{}", content);
}
```

## Example Project

See the `example/` directory for a working demonstration.

```bash
cd example
cargo run
```

## License

MIT
