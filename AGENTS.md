# AGENTS.md — uapi-sdk-rust

This file tells AI coding agents how to use the **official Rust SDK** for
the [uapis.cn](https://uapis.cn) public API platform.

## What this crate is

Idiomatic, typed Rust client for UAPI. Generated from the live OpenAPI 3.1
spec at <https://uapis.cn/openapi.json> — method names, parameter shapes,
and return types track the real API.

```toml
[dependencies]
uapi-sdk-rust = "0.1"
```

## Quick start

```rust
use uapi_sdk_rust::{Client, GetMiscWeatherRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Free-tier endpoints don't need a key
    let client = Client::new("https://uapis.cn");
    let weather = client
        .misc()
        .get_misc_weather(&GetMiscWeatherRequest { city: "北京".into() })
        .await?;
    println!("{:#?}", weather);
    Ok(())
}
```

The client is grouped by tag (`misc()`, `network()`, `text()`, `image()`,
`social()`, `translate()`, `search()`, …). Method names match the OpenAPI
`operationId`, snake-cased.

## Authentication

Free-tier endpoints work with no key. Paid endpoints take a key:

```rust
let client = Client::new("https://uapis.cn").with_api_key("sk_…");
```

## Errors

Every method returns `Result<T, uapi_sdk_rust::Error>`. The error variants
map cleanly to UAPI's `{code, success: false, error, request_id?}`. Surface
the `error` text verbatim.

## Rate limits

Headers `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`,
`Retry-After` are exposed on responses. Honor them — back off on `429`.

## TLS features

```toml
# Default: rustls
uapi-sdk-rust = "0.1"

# Or use platform native TLS:
uapi-sdk-rust = { version = "0.1", default-features = false, features = ["native-tls"] }
```

## Related repos

- MCP server: <https://github.com/AxT-Team/uapi-mcp>.
- Skills bundle: <https://github.com/AxT-Team/uapi-agent-skills>.
- Other languages: `uapi-sdk-typescript`, `uapi-sdk-python`, `uapi-sdk-go`,
  `uapi-sdk-java`, `uapi-sdk-csharp`, `uapi-sdk-cpp`, `uapi-sdk-php`.
