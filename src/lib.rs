//! This crate provides Rust bindings to the Treeland wayland protocol extensions
//! provided in <https://github.com/AkiraMiyak662/treeland-protocols>
//!
//! Treeland is a Wayland compositor developed by UnionTech/Deepin, these protocols
//! are used by DDE (Deepin Desktop Environment) components.
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.
//!
//! ## Protocol Categories
//!
//! The protocols provided in this crate include:
//!
//! - [`app_id_resolver`] - Application ID resolver for window identification
//! - [`capture`] - Screen/window capture functionality
//! - [`dde_shell`] - DDE shell integration (multitask view, window picker, etc.)
//! - [`ddm`] - DDM (Display Device Manager) protocol
//! - [`foreign_toplevel_manager`] - Foreign toplevel window management
//! - [`output_manager`] - Output/display management
//! - [`personalization_manager`] - Desktop personalization (wallpaper, cursor, theme)
//! - [`prelaunch_splash`] - Pre-launch splash screen
//! - [`screensaver`] - Screensaver control
//! - [`shortcut_manager`] - Global shortcut registration
//! - [`virtual_output_manager`] - Virtual output management
//! - [`wallpaper_color`] - Wallpaper color extraction
//! - [`window_management`] - Window management operations
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wayland-protocols-treeland = { version = "0.1", features = ["client"] }
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! use wayland_protocols_treeland::shortcut_manager::v1::client::treeland_shortcut_manager_v1;
//!
//! // Register a global shortcut
//! let shortcut_manager: treeland_shortcut_manager_v1::TreelandShortcutManagerV1 = ...;
//! let context = shortcut_manager.register_shortcut_context("Ctrl+Alt+T", &qh, ());
//! ```

#![warn(missing_docs)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

/// Application ID resolver protocol
///
/// This protocol allows clients to resolve application IDs for windows.
pub mod app_id_resolver {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-app-id-resolver-v1.xml",
            []
        );
    }
}

/// Screen/window capture protocol (unstable)
///
/// This protocol provides screen and window capture functionality.
pub mod capture {
    /// Unstable version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-capture-unstable-v1.xml",
            []
        );
    }
}

/// DDE Shell protocol
///
/// This protocol allows DDE shell components to interact with Treeland,
/// including features like:
/// - Window overlap detection
/// - Shell surfaces
/// - Multi-task view
/// - Window picker
/// - Lock screen
pub mod dde_shell {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-dde-shell-v1.xml",
            []
        );
    }
}

/// DDM (Display Device Manager) protocol
///
/// This protocol manages display devices.
/// NOTE: This protocol has structural issues and requires fixes from upstream
// pub mod ddm {
//     /// Version 1 of the protocol
//     pub mod v1 {
//         wayland_protocol!(
//             "./treeland-protocols/xml/treeland-ddm-v1.xml",
//             []
//         );
//     }
// }

/// Foreign toplevel manager protocol
///
/// This protocol allows clients to get information about and interact with
/// toplevel windows from other applications. Useful for taskbars, docks,
/// and window management utilities.
pub mod foreign_toplevel_manager {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-foreign-toplevel-manager-v1.xml",
            []
        );
    }
}

/// Output manager protocol
///
/// This protocol provides output/display management functionality.
pub mod output_manager {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-output-manager-v1.xml",
            []
        );
    }
}

/// Personalization manager protocol
///
/// This protocol allows clients to customize display effects, including:
/// - Window background and shadow
/// - Wallpaper settings
/// - Cursor theme
/// - Font settings
/// - Appearance settings
pub mod personalization_manager {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-personalization-manager-v1.xml",
            []
        );
    }
}

/// Pre-launch splash protocol
///
/// This protocol provides splash screen functionality for application launch.
pub mod prelaunch_splash {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-prelaunch-splash-v1.xml",
            []
        );
    }
}

/// Screensaver protocol
///
/// This protocol allows clients to control the screensaver and inhibit idle.
/// NOTE: This protocol has structural issues and requires fixes from upstream
// pub mod screensaver {
//     /// Version 1 of the protocol
//     pub mod v1 {
//         wayland_protocol!(
//             "./treeland-protocols/xml/treeland-screensaver-v1.xml",
//             []
//         );
//     }
// }

/// Shortcut manager protocol
///
/// This protocol allows clients to register global keyboard shortcuts.
pub mod shortcut_manager {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-shortcut-manager-v1.xml",
            []
        );
    }

    /// Version 2 of the protocol
    pub mod v2 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-shortcut-manager-v2.xml",
            []
        );
    }
}

/// Virtual output manager protocol
///
/// This protocol provides virtual output management functionality.
pub mod virtual_output_manager {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-virtual-output-manager-v1.xml",
            []
        );
    }
}

/// Wallpaper color protocol
///
/// This protocol allows clients to get dominant colors from the wallpaper.
pub mod wallpaper_color {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-wallpaper-color-v1.xml",
            []
        );
    }
}

/// Window management protocol
///
/// This protocol provides window management operations.
pub mod window_management {
    /// Version 1 of the protocol
    pub mod v1 {
        wayland_protocol!(
            "./treeland-protocols/xml/treeland-window-management-v1.xml",
            []
        );
    }
}
