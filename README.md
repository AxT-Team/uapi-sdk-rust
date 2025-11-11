# uapi-sdk-rust

![Banner](https://raw.githubusercontent.com/AxT-Team/uapi-sdk-rust/main/banner.png)

[![Rust](https://img.shields.io/badge/Rust-1.75+-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/Docs-uapis.cn-2EAE5D?style=flat-square)](https://uapis.cn/)

> [!NOTE]
> 所有接口的 Rust 示例都可以在 [UApi](https://uapis.cn/docs/introduction) 的接口文档页面，向下滚动至 **快速启动** 区块后直接复制。

## 快速开始

```bash
cargo add uapi-sdk-rust
```

```rust
use _::Client;

#[tokio::main]
async fn main() -> Result<(), _::Error> {
    let client = Client::new("");
    let result = client.social().get_social_qq_userinfo("10001").await?;
    println!("{result:?}");
    Ok(())
}
```

## 特性

现在你不再需要反反复复的查阅文档了。

只需在 IDE 中键入 `client.`，所有核心模块——如 `social()`、`game()`、`image()`——即刻同步展现。进一步输入即可直接定位到 `get_social_qq_userinfo` 这样的具体方法，其名称与文档的 `operationId` 严格保持一致，确保了开发过程的直观与高效。

所有方法签名只接受真实且必需的参数。当你在构建请求时，IDE 会即时提示 `qq`、`username` 等键名，这彻底杜绝了在 `&str` / `serde_json::Value` 等常规类型中因键名拼写错误而导致的运行时错误。

针对 401、404、429 等标准 HTTP 响应，SDK 已将其统一映射为具名的错误类型（`Error::AuthenticationError`、`Error::NotFound`、`Error::RateLimitError` 等）。这些错误均附带 `status()`、`request_id()`、`details()` 等关键上下文信息，确保你在日志中能第一时间准确、快速地诊断问题。

`Client::builder()` / `Client::new()` / `Client::from_env()` 允许你在保持默认 15 秒超时与 `Authorization` 头的同时，自由覆盖 Base URL、Token、代理乃至注入自定义的 `reqwest::Client`。

如果你需要查看字段细节或内部逻辑，仓库中的 `./internal` 目录同步保留了由 `openapi-generator` 生成的完整结构体，随时可供参考。

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



