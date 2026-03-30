//! Platform-specific file paths for Flux VPN
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


use std::path::PathBuf;

/// Returns the settings directory for the current platform.
pub fn settings_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/etc/flux-vpn/");
    #[cfg(target_os = "macos")]
    return PathBuf::from("/etc/flux-vpn/");
    #[cfg(target_os = "windows")]
    return dirs::data_local_dir().unwrap().join("Flux VPN");
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return PathBuf::from("./flux-vpn/");
}

/// Returns the log directory for the current platform.
pub fn log_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/log/flux-vpn/");
    #[cfg(target_os = "macos")]
    return PathBuf::from("/var/log/flux-vpn/");
    #[cfg(target_os = "windows")]
    return PathBuf::from("C:\\ProgramData\\Flux VPN\\");
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return PathBuf::from("./logs/");
}

/// Returns the cache directory for the current platform.
pub fn cache_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    return PathBuf::from("/var/cache/flux-vpn/");
    #[cfg(target_os = "macos")]
    return PathBuf::from("/Library/Caches/flux-vpn/");
    #[cfg(target_os = "windows")]
    return PathBuf::from("C:\\ProgramData\\Flux VPN\\cache");
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return PathBuf::from("./cache/");
}

