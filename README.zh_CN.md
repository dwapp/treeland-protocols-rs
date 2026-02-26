# Treeland 协议 Rust 绑定

[English](README.md) | [简体中文](README.zh_CN.md)

为 [Treeland Wayland 协议扩展](https://github.com/linuxdeepin/treeland-protocols)提供的 Rust 绑定。

这些绑定基于 [wayland-client](https://crates.io/crates/wayland-client) 和 [wayland-server](https://crates.io/crates/wayland-server) 构建。

关于协议本身的详细信息，请参阅上游仓库：https://github.com/linuxdeepin/treeland-protocols

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
wayland-protocols-treeland = { git = "https://github.com/dwapp/treeland-protocols-rs" }
```

## 功能特性

- `client`（默认）：启用客户端绑定
- `server`：启用服务端绑定

```toml
# 仅客户端（默认）
wayland-protocols-treeland = { git = "..." }

# 仅服务端
wayland-protocols-treeland = { git = "...", default-features = false, features = ["server"] }

# 客户端和服务端都启用
wayland-protocols-treeland = { git = "...", features = ["client", "server"] }
```

## 构建

```bash
# 克隆仓库（包含子模块）
git clone --recursive https://github.com/dwapp/treeland-protocols-rs

cd treeland-protocols-rs

cargo build
```

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

`treeland-protocols/` 中的协议定义单独授权，详见该仓库。
