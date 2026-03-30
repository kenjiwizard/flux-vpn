//! Talpid type definitions
//!
//! Part of the Talpid VPN engine — a generic, privacy-preserving VPN client library.
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


use serde::{Deserialize, Serialize};

/// Error type for tunnel operations.
#[derive(Debug, thiserror::Error)]
pub enum ErrorStateCause {
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Firewall policy error")]
    SetFirewallPolicyError,
    #[error("DNS error")]
    SetDnsError,
    #[error("Tunnel parameter error")]
    TunnelParameterError,
}

/// Network tunnel endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: String,
    pub port: u16,
    pub protocol: TransportProtocol,
}

/// Supported transport protocols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportProtocol {
    Tcp,
    Udp,
}

