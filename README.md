# Logs 日志库

一个 Rust 日志库，提供基于日期的日志文件轮转功能。

## 功能特性

- 基于日期的自动日志文件轮转
- 控制台和文件输出
- 可配置的日志级别
- 简单的初始化

## 使用方法

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
logs = { path = "../path/to/logs" }
or
logs = { git = "ssh://git@github.com/RustFlux/logs.git", branch = "main" }

```

在代码中初始化和使用日志：

```rust
use logs::{init_logging, test_logging};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    init_logging()?;
    
    // 测试日志功能
    test_logging();
    
    // 自定义日志
    log::info!("应用程序已启动");
    
    Ok(())
}
```

## 配置说明

日志文件会自动创建在 `logs/` 目录下，文件名遵循 `app_YYYY-MM-DD.log` 的格式。

## 示例输出

```
2025-01-09T12:34:56.789 INFO  main - 应用程序已启动
2025-01-09T12:34:56.790 DEBUG main - 这是一个调试信息 0
2025-01-09T12:34:56.790 INFO  main - 这是一个普通信息 0
2025-01-09T12:34:56.790 WARN  main - 这是一个警告信息 0
2025-01-09T12:34:56.790 ERROR main - 这是一个错误信息 0
```

## 许可证

MIT
