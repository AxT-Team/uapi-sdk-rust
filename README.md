# uapi-sdk-rust

![Banner](https://raw.githubusercontent.com/AxT-Team/uapi-sdk-rust/main/banner.png)

[![Rust](https://img.shields.io/badge/Rust-1.75+-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/Docs-uapis.cn-2EAE5D?style=flat-square)](https://uapis.cn/)
[![crates.io](https://img.shields.io/crates/v/uapi-sdk-rust?style=flat-square&logo=rust)](https://crates.io/crates/uapi-sdk-rust)
[![docs.rs](https://img.shields.io/docsrs/uapi-sdk-rust?label=docs.rs&style=flat-square)](https://docs.rs/uapi-sdk-rust)

> [!NOTE]
> 所有接口的 Rust 示例都可以在 [UApi](https://uapis.cn/docs/introduction) 的接口文档页面，向下滚动至 **快速启动** 区块后直接复制。

## 快速开始

```bash
cargo add uapi-sdk-rust
```

```rust
use uapi_sdk_rust::{Client, Result};
use uapi_sdk_rust::services::GetMiscHotboardParams;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY");
    let result = client.misc().get_misc_hotboard(GetMiscHotboardParams::new("weibo")).await?;
    println!("{result:?}");
    Ok(())
}
```

这个接口默认只要传 `type` 就可以拿当前热榜。`time_query`、`keyword_query`、`time_start_query`、`time_end_query`、`limit_query`、`sources_query` 都是按场景再传的可选参数。

## 特性

现在你不再需要反反复复的查阅文档了。

只需在 IDE 中键入 `client.`，所有核心模块——如 `social()`、`game()`、`image()`——即刻同步展现。进一步输入即可直接定位到 `get_social_qq_userinfo` 这样的具体方法，其名称与文档的 `operationId` 严格保持一致，确保了开发过程的直观与高效。

所有方法签名只接受真实且必需的参数。当你在构建请求时，IDE 会即时提示 `qq`、`username` 等键名，这彻底杜绝了在 `&str` / `serde_json::Value` 等常规类型中因键名拼写错误而导致的运行时错误。

针对 401、404、429 等标准 HTTP 响应，SDK 已将其统一映射为具名的错误类型（`Error::AuthenticationError`、`Error::NotFound`、`Error::RateLimitError` 等）。这些错误均附带 `status()`、`request_id()`、`details()` 等关键上下文信息，确保你在日志中能第一时间准确、快速地诊断问题。

`Client::builder()` / `Client::new()` / `Client::from_env()` 允许你在保持默认 15 秒超时与 `Authorization` 头的同时，自由覆盖 Base URL、Token、代理乃至注入自定义的 `reqwest::Client`。

如果你需要查看字段细节或内部逻辑，仓库中的 `./internal` 目录同步保留了由 `openapi-generator` 生成的完整结构体，随时可供参考。

## 响应元信息

每次请求完成后，SDK 会自动把响应 Header 解析成结构化的 `ResponseMeta`，你不用自己拆原始字符串。

成功时可以通过 `client.last_response_meta()` 读取，失败时可以通过 `e.meta()` 读取，两条路径拿到的是同一套字段。

```rust
use uapi_sdk_rust::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY");

    // 成功路径
    let _result = client.social().get_social_qq_userinfo("10001").await?;
    if let Some(meta) = client.last_response_meta() {
        println!("余额剩余: {} 分", meta.balance_remaining_cents.unwrap_or(0));
        println!("资源包剩余: {} 积分", meta.quota_remaining_credits.unwrap_or(0));
        println!("Request ID: {:?}", meta.request_id);
    }

    // 失败路径
    match client.social().get_social_qq_userinfo("10001").await {
        Err(e) => {
            if let Some(meta) = e.meta() {
                println!("限流，{}s 后重试", meta.retry_after_seconds.unwrap_or(0));
                println!("Request ID: {:?}", meta.request_id);
            }
        }
        Ok(_) => {}
    }

    Ok(())
}
```

常用字段一览：

| 字段 | 说明 |
|------|------|
| `balance_remaining_cents` | 账户余额剩余（分） |
| `quota_remaining_credits` | 资源包剩余积分 |
| `visitor_quota_remaining_credits` | 访客配额剩余积分 |
| `retry_after_seconds` | 触发限流后的建议等待时长 |
| `request_id` | 请求唯一 ID，排障时使用 |
| `debit_status` | 本次计费状态 |
| `rate_limit_policies` / `rate_limits` | 完整结构化限流策略数据 |

## 模型

`uapi::models` 直接 re-export 了 `internal/src/models` 中的所有结构体，因此你可以在 IDE 中获得与 OpenAPI 保持一致的强类型定义。例如：

```rust
use uapi::{Client, models::GetNetworkIpinfo200Response};
```

所有模型都派生了 `Serialize` / `Deserialize`，可以安全地持久化或在应用调用间传递。

## 错误模型概览

| HTTP 状态码 | SDK 错误类型             | 附加信息                                          |
|-------------|-------------------------|---------------------------------------------------|
| 401/403     | `Error::AuthenticationError` | `status`、`request_id`                             |
| 404         | `Error::NotFound`       | `status`、`request_id`                             |
| 400         | `Error::ValidationError`| `status`、`details`、`request_id`                  |
| 429         | `Error::RateLimitError` | `status`、`retry_after_seconds`、`request_id`      |
| 5xx         | `Error::ServerError`    | `status`、`request_id`                             |
| 其他 4xx/5xx| `Error::ApiError`       | `code`、`status`、`details`、`request_id`          |

## 其他 SDK

| 语言        | 仓库地址                                                     |
|-------------|--------------------------------------------------------------|
| Go          | https://github.com/AxT-Team/uapi-sdk-go                      |
| Python      | https://github.com/AxT-Team/uapi-sdk-python                  |
| TypeScript| https://github.com/AxT-Team/uapi-sdk-typescript           |
| Browser (TypeScript/JavaScript)| https://github.com/AxT-Team/uapi-browser-sdk        |
| Java        | https://github.com/AxT-Team/uapi-sdk-java                    |
| PHP         | https://github.com/AxT-Team/uapi-sdk-php                     |
| C#          | https://github.com/AxT-Team/uapi-sdk-csharp                  |
| C++         | https://github.com/AxT-Team/uapi-sdk-cpp                     |
| Rust（当前）        | https://github.com/AxT-Team/uapi-sdk-rust                    |

## 文档

访问 [UApi文档首页](https://uapis.cn/docs/introduction) 并选择任意接口，向下滚动到 **快速启动** 区块即可看到最新的 Rust 示例代码。



