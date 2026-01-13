# Treeland 协议 Rust 绑定

[English](README.md) | [简体中文](README.zh_CN.md)

为 Treeland Wayland 协议扩展提供的 Rust 绑定。

## 概述

本项目为 Treeland Wayland 协议扩展提供 Rust 绑定。[Treeland](https://github.com/linuxdeepin/treeland) 是由统信软件/深度开发的用于 DDE（深度桌面环境）的 Wayland 合成器。

这些绑定基于 [wayland-client](https://crates.io/crates/wayland-client) 和 [wayland-server](https://crates.io/crates/wayland-server) 构建。

## 项目结构

```
treeland-protocols-rs/
├── src/                      # Rust 绑定源代码
│   ├── lib.rs               # 主库入口
│   └── protocol_macro.rs    # 协议生成宏
├── treeland-protocols/      # Git 子模块 - 协议定义
│   └── xml/                 # XML 协议文件
├── Cargo.toml
└── README.md
```

## 支持的协议

| 协议 | 状态 | 描述 |
|------|------|------|
| `app_id_resolver` | ✅ | 窗口应用 ID 解析器 |
| `capture` | ✅ | 屏幕/窗口捕获 |
| `dde_shell` | ✅ | DDE 壳层集成（多任务视图、窗口选择器等） |
| `ddm` | ✅ | 显示设备管理器 |
| `foreign_toplevel_manager` | ✅ | 外部顶层窗口管理 |
| `output_manager` | ✅ | 输出/显示管理 |
| `personalization_manager` | ✅ | 桌面个性化（壁纸、光标、主题） |
| `prelaunch_splash` | ✅ | 预启动启动画面 |
| `screensaver` | ✅ | 屏保控制 |
| `shortcut_manager` | ✅ | 全局快捷键注册（v1 & v2） |
| `virtual_output_manager` | ✅ | 虚拟输出管理 |
| `wallpaper_color` | ✅ | 壁纸颜色提取 |
| `window_management` | ✅ | 窗口管理操作 |

所有列出的协议均已启用并通过验证；如果上游协议定义发生更改，会在此处更新。

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

## 使用示例

### 注册全局快捷键

```rust
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols_treeland::shortcut_manager::v1::client::{
    treeland_shortcut_context_v1,
    treeland_shortcut_manager_v1,
};

struct AppData;

impl Dispatch<treeland_shortcut_manager_v1::TreelandShortcutManagerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &treeland_shortcut_manager_v1::TreelandShortcutManagerV1,
        _event: treeland_shortcut_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // 处理管理器事件
    }
}

impl Dispatch<treeland_shortcut_context_v1::TreelandShortcutContextV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &treeland_shortcut_context_v1::TreelandShortcutContextV1,
        event: treeland_shortcut_context_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            treeland_shortcut_context_v1::Event::Shortcut => {
                println!("快捷键被触发！");
            }
            _ => {}
        }
    }
}
```

### 窗口管理

```rust
use wayland_protocols_treeland::foreign_toplevel_manager::v1::client::{
    treeland_foreign_toplevel_handle_v1,
    treeland_foreign_toplevel_manager_v1,
};

// 获取所有顶层窗口的信息
// 适用于任务栏和窗口管理工具
```

### 个性化（壁纸）

```rust
use wayland_protocols_treeland::personalization_manager::v1::client::{
    treeland_personalization_manager_v1,
    treeland_personalization_wallpaper_context_v1,
};

// 为不同输出设置自定义壁纸
```

## 构建

```bash
# 克隆仓库（包含子模块）
git clone --recursive https://github.com/dwapp/treeland-protocols-rs

cd treeland-protocols-rs

# 构建
cargo build

# 运行测试
cargo test

# 检查所有功能特性
cargo check --all-features
```

## 贡献

欢迎贡献！请注意：

1. `treeland-protocols/` 目录是指向上游协议定义的 git 子模块
2. 协议 bug 应报告到 [treeland-protocols](https://github.com/dwapp/treeland-protocols) 仓库
3. 绑定问题可以在此处报告

## 相关项目

- [treeland](https://github.com/linuxdeepin/treeland) - Treeland Wayland 合成器
- [treeland-protocols](https://github.com/linuxdeepin/treeland-protocols) - 协议定义
- [wayland-rs](https://github.com/smithay/wayland-rs) - Rust Wayland 绑定

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

`treeland-protocols/` 中的协议定义单独授权，详见该仓库。
