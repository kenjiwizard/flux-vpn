//! Flux VPN API client
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


/// API client for communicating with Flux VPN services.
pub struct FluxApiClient {
    base_url: String,
}

impl FluxApiClient {
    pub fn new(base_url: &str) -> Self {
        Self { base_url: base_url.to_string() }
    }
}

