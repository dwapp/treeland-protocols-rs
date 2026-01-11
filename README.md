# Treeland Protocols Rust Bindings

[English](README.md) | [简体中文](README.zh_CN.md)

Rust bindings for Treeland Wayland protocol extensions.

## Overview

This crate provides Rust bindings to the Treeland Wayland protocol extensions. [Treeland](https://github.com/linuxdeepin/treeland) is a Wayland compositor developed by UnionTech/Deepin for DDE (Deepin Desktop Environment).

These bindings are built on top of [wayland-client](https://crates.io/crates/wayland-client) and [wayland-server](https://crates.io/crates/wayland-server).

## Project Structure

```
treeland-protocols-rs/
├── src/                      # Rust binding source code
│   ├── lib.rs               # Main library entry
│   └── protocol_macro.rs    # Protocol generation macro
├── treeland-protocols/      # Git submodule - protocol definitions
│   └── xml/                 # XML protocol files
├── example/                 # Reference implementations
│   ├── wayland-rs/         # Official wayland-rs bindings
│   └── wayland-protocols-hyprland-rs/  # Hyprland protocol bindings
├── Cargo.toml
└── README.md
```

## Supported Protocols

| Protocol | Status | Description |
|----------|--------|-------------|
| `app_id_resolver` | ✅ | Application ID resolver for window identification |
| `capture` | ⚠️  | Screen/window capture (has upstream bug) |
| `dde_shell` | ✅ | DDE shell integration (multitask view, window picker, etc.) |
| `ddm` | ⚠️  | Display Device Manager (non-standard naming) |
| `foreign_toplevel_manager` | ✅ | Foreign toplevel window management |
| `output_manager` | ✅ | Output/display management |
| `personalization_manager` | ✅ | Desktop personalization (wallpaper, cursor, theme) |
| `prelaunch_splash` | ✅ | Pre-launch splash screen |
| `screensaver` | ⚠️  | Screensaver control (non-standard naming) |
| `shortcut_manager` | ✅ | Global shortcut registration (v1 & v2) |
| `virtual_output_manager` | ✅ | Virtual output management |
| `wallpaper_color` | ✅ | Wallpaper color extraction |
| `window_management` | ✅ | Window management operations |

**Note**: Protocols marked with ⚠️ are currently disabled due to issues in the upstream protocol definitions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wayland-protocols-treeland = { git = "https://github.com/AkiraMiyak662/treeland-protocols-rs" }
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

## Usage Example

### Registering a Global Shortcut

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
        // Handle manager events
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
                println!("Shortcut triggered!");
            }
            _ => {}
        }
    }
}
```

### Getting Wallpaper Color

```rust
use wayland_protocols_treeland::wallpaper_color::v1::client::treeland_wallpaper_color_manager_v1;

// After binding the global...
// let color_manager = globals.bind::<treeland_wallpaper_color_manager_v1::TreelandWallpaperColorManagerV1, _, _>(...);
```

## Building

```bash
# Clone with submodules
git clone --recursive https://github.com/AkiraMiyak662/treeland-protocols-rs

cd treeland-protocols-rs

# Build
cargo build

# Run tests
cargo test

# Check all features
cargo check --all-features
```

## Contributing

Contributions are welcome! Please note:

1. The `treeland-protocols/` directory is a git submodule pointing to the upstream protocol definitions
2. Protocol bugs should be reported to the [treeland-protocols](https://github.com/linuxdeepin/treeland-protocols) repository
3. Binding issues can be reported here

## Related Projects

- [treeland](https://github.com/linuxdeepin/treeland) - The Treeland Wayland compositor
- [treeland-protocols](https://github.com/linuxdeepin/treeland-protocols) - Protocol definitions
- [wayland-rs](https://github.com/smithay/wayland-rs) - Rust Wayland bindings
- [wayland-protocols-wlr](https://crates.io/crates/wayland-protocols-wlr) - WLR protocol bindings
- [wayland-protocols-plasma](https://crates.io/crates/wayland-protocols-plasma) - KDE Plasma protocol bindings

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The protocol definitions in `treeland-protocols/` are licensed separately, see that repository for details.
