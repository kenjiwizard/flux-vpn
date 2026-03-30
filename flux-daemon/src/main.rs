//! Flux VPN Daemon
//!
//! The main system service that manages VPN tunnels, firewall rules,
//! and exposes a management interface for frontends (GUI and CLI).
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting Flux VPN daemon v{}", env!("CARGO_PKG_VERSION"));

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async {
        log::info!("Flux VPN daemon is running");
        // TODO: Initialize tunnel manager, firewall, and management interface
        tokio::signal::ctrl_c().await.ok();
        log::info!("Shutting down Flux VPN daemon");
    });

    Ok(())
}
