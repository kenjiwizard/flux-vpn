//! Logging utilities for Flux VPN
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji
//! License: GPL-3.0-or-later


/// Initialize the Flux VPN logger with the given level.
pub fn init_logger(level: log::LevelFilter) {
    let _ = env_logger::builder().filter_level(level).try_init();
}

