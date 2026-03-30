//! DNS configuration for Flux VPN
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


/// DNS configuration module.
pub enum DnsMethod {
    StaticFile,
    Resolvconf,
    Systemd,
    NetworkManager,
}

