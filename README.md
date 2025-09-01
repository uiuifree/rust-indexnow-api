# IndexNow API for Rust

[![Crates.io](https://img.shields.io/crates/v/indexnow-api.svg)](https://crates.io/crates/indexnow-api)
[![Documentation](https://docs.rs/indexnow-api/badge.svg)](https://docs.rs/indexnow-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for instantly notifying search engines about URL changes using the IndexNow protocol. Supports Bing, Yandex, and other IndexNow-compatible search engines.

## Features

- 🚀 **Async/await support** - Built with modern Rust async patterns
- 🔧 **Simple API** - Easy to integrate with just a few lines of code
- 🌐 **Multiple search engines** - Works with Bing, Yandex, and other IndexNow endpoints
- 🛡️ **Error handling** - Comprehensive error types for robust applications
- 📦 **Zero-config** - Works out of the box with sensible defaults

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
indexnow-api = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Usage

### Basic Example

```rust
use indexnow_api::IndexNowApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with your domain and API key
    let api = IndexNowApi::new(
        "www.example.com",
        "7be9fca90b3b4b039983fa8f06e03ee8"
    );

    // Notify search engines about URL changes
    let urls = vec!["https://www.example.com/new-page".to_string()];
    api.send_urls(urls).await?;

    println!("URLs successfully submitted to IndexNow!");
    Ok(())
}
```

### Batch URL Submission

Submit multiple URLs at once for better efficiency:

```rust
use indexnow_api::IndexNowApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = IndexNowApi::new(
        "www.example.com",
        "7be9fca90b3b4b039983fa8f06e03ee8"
    );

    // Submit up to 10,000 URLs per request
    let urls = vec![
        "https://www.example.com/page1".to_string(),
        "https://www.example.com/page2".to_string(),
        "https://www.example.com/updated-article".to_string(),
    ];

    api.send_urls(urls).await?;
    Ok(())
}
```

### Custom Search Engine Endpoint

Target specific search engines by setting custom endpoints:

```rust
use indexnow_api::IndexNowApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = IndexNowApi::new(
        "www.example.com",
        "7be9fca90b3b4b039983fa8f06e03ee8"
    );

    // Use Bing's dedicated endpoint for faster processing
    api.set_search_engine("https://www.bing.com");
    
    // Optional: specify custom key file location
    api.set_key_location("https://www.example.com/my-indexnow-key.txt");

    let urls = vec!["https://www.example.com/important-update".to_string()];
    api.send_urls(urls).await?;

    Ok(())
}
```

### Error Handling

Handle different types of errors gracefully:

```rust
use indexnow_api::{IndexNowApi, GoogleApiError};

#[tokio::main]
async fn main() {
    let api = IndexNowApi::new("www.example.com", "your-api-key");
    
    let urls = vec!["https://www.example.com/page".to_string()];
    
    match api.send_urls(urls).await {
        Ok(()) => println!("✅ URLs successfully submitted"),
        Err(GoogleApiError::Connection(msg)) => {
            eprintln!("🔗 Connection error: {}", msg);
        }
        Err(GoogleApiError::Status(code)) => {
            eprintln!("⚠️  HTTP error with status: {}", code);
        }
    }
}
```

## API Reference

### `IndexNowApi`

The main struct for interacting with IndexNow APIs.

#### Constructor

```rust
pub fn new<T: ToString, U: ToString>(host: T, key: U) -> IndexNowApi
```

**Parameters:**
- `host` - Your website's hostname (e.g., "www.example.com")
- `key` - Your IndexNow API key (32-character hexadecimal string)

#### Methods

##### `set_search_engine`
```rust
pub fn set_search_engine<T: ToString>(&mut self, search_engine: T)
```
Sets a custom search engine endpoint. Default: `https://api.indexnow.org`

**Popular endpoints:**
- Bing: `https://www.bing.com`
- Yandex: `https://yandex.com`

##### `set_key_location`
```rust
pub fn set_key_location<T: ToString>(&mut self, key_location: T)
```
Specifies where your API key file is hosted (optional).

##### `send_urls`
```rust
pub async fn send_urls(&self, urls: Vec<String>) -> Result<(), GoogleApiError>
```
Submits a list of URLs to the search engine for indexing.

**Limits:**
- Maximum 10,000 URLs per request
- URLs must be from the same domain as the `host`

## Getting Your IndexNow API Key

### Step 1: Generate an API Key

Create a 32-character hexadecimal key:

```bash
# Using OpenSSL
openssl rand -hex 16

# Using Python
python -c "import secrets; print(secrets.token_hex(16))"

# Example output: 7be9fca90b3b4b039983fa8f06e03ee8
```

### Step 2: Host the Key File

Create a text file containing only your API key and host it at:
```
https://yourdomain.com/[your-api-key].txt
```

**Example:**
- API Key: `7be9fca90b3b4b039983fa8f06e03ee8`
- File location: `https://www.example.com/7be9fca90b3b4b039983fa8f06e03ee8.txt`
- File contents: `7be9fca90b3b4b039983fa8f06e03ee8`

### Step 3: Verify Setup

Test your setup by submitting a URL and checking for successful responses.

## Supported Search Engines

| Search Engine | Endpoint | Status |
|---------------|----------|--------|
| Bing | `https://www.bing.com/indexnow` | ✅ Supported |
| Yandex | `https://yandex.com/indexnow` | ✅ Supported |
| Generic | `https://api.indexnow.org/indexnow` | ✅ Supported |

## Best Practices

1. **Batch requests** - Submit multiple URLs in a single request when possible
2. **Rate limiting** - Don't submit the same URL multiple times within 24 hours
3. **Key security** - Keep your API key secure and rotate it periodically
4. **Error handling** - Always handle potential network and API errors
5. **URL validation** - Ensure URLs belong to your verified domain

## Error Types

### `GoogleApiError::Connection(String)`
Network connectivity issues, DNS problems, or server timeouts.

### `GoogleApiError::Status(u8)`
HTTP status codes indicating API errors (4xx, 5xx responses).

## Examples

Check out the `tests/` directory for more usage examples:

```bash
cargo test -- --nocapture
```

## Contributing

We welcome contributions! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup

```bash
git clone https://github.com/uiuifree/rust-indexnow-api.git
cd rust-indexnow-api
cargo test
```

### Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with tests
4. Ensure `cargo test` passes
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Resources

- [IndexNow Protocol Documentation](https://www.indexnow.org/)
- [Microsoft Bing IndexNow](https://www.bing.com/indexnow)
- [Yandex IndexNow](https://yandex.com/support/webmaster/indexnow.html)

## Changelog

### v0.1.0
- Initial release
- Basic IndexNow API support
- Async/await implementation
- Error handling
- Multiple search engine endpoints

---

**Questions?** Open an issue on [GitHub](https://github.com/uiuifree/rust-indexnow-api) or check the [documentation](https://docs.rs/indexnow-api).
