//! Traffic obfuscation protocols
//!
//! Part of the Talpid VPN engine — a generic, privacy-preserving VPN client library.
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


/// Obfuscation methods for tunneling WireGuard through restrictive networks.
pub enum ObfuscationProtocol {
    /// WireGuard over TCP
    Tcp,
    /// WireGuard over Shadowsocks
    Shadowsocks,
    /// WireGuard over QUIC
    Quic,
}

