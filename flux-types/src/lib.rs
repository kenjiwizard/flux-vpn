//! Shared type definitions for Flux VPN
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


use serde::{Deserialize, Serialize};

/// Represents the current tunnel state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TunnelState {
    Disconnected,
    Connecting,
    Connected(ConnectionInfo),
    Disconnecting,
    Error(String),
}

/// Information about an active connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub server: String,
    pub protocol: String,
    pub ip: String,
    pub port: u16,
}

/// A VPN relay server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relay {
    pub hostname: String,
    pub country: String,
    pub city: String,
    pub ip_v4: String,
    pub ip_v6: Option<String>,
    pub weight: u32,
    pub active: bool,
}

