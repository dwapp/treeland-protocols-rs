# Treeland Protocols Rust Bindings

[English](README.md) | [简体中文](README.zh_CN.md)

Rust bindings for [Treeland Wayland protocol extensions](https://github.com/linuxdeepin/treeland-protocols).

These bindings are built on top of [wayland-client](https://crates.io/crates/wayland-client) and [wayland-server](https://crates.io/crates/wayland-server).

For details about the protocols themselves, please refer to the upstream repository: https://github.com/linuxdeepin/treeland-protocols

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wayland-protocols-treeland = { git = "https://github.com/dwapp/treeland-protocols-rs" }
```

## Features

- `client` (default): Enable client-side bindings
- `server`: Enable server-side bindings

```toml
# Client only (default)
wayland-protocols-treeland = { git = "..." }

# Server only
wayland-protocols-treeland = { git = "...", default-features = false, features = ["server"] }

# Both client and server
wayland-protocols-treeland = { git = "...", features = ["client", "server"] }
```

## Building

```bash
# Clone with submodules
git clone --recursive https://github.com/dwapp/treeland-protocols-rs

cd treeland-protocols-rs

cargo build
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The protocol definitions in `treeland-protocols/` are licensed separately, see that repository for details.
