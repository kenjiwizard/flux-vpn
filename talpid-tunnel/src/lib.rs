//! Tunnel abstraction layer
//!
//! Part of the Talpid VPN engine — a generic, privacy-preserving VPN client library.
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


/// Abstract tunnel interface for different VPN protocols.
pub trait Tunnel: Send {
    fn get_interface_name(&self) -> &str;
    fn stop(self: Box<Self>) -> std::result::Result<(), String>;
}

