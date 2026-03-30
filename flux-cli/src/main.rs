//! Flux VPN CLI
//!
//! Terminal-based frontend for controlling the Flux VPN daemon.
//!
//! Copyright (C) 2026 Flux VPN — Created by Kenji

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "flux")]
#[command(about = "Flux VPN command-line client")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a VPN server
    Connect {
        /// Server location (e.g., "us-nyc", "de-ber")
        #[arg(short, long)]
        location: Option<String>,
    },
    /// Disconnect from the VPN
    Disconnect,
    /// Show connection status
    Status,
    /// List available relay servers
    Relays,
    /// Manage split tunneling
    SplitTunnel {
        #[command(subcommand)]
        action: SplitTunnelAction,
    },
    /// Manage DNS settings
    Dns {
        /// Set a custom DNS server
        #[arg(short, long)]
        set: Option<String>,
    },
}

#[derive(Subcommand)]
enum SplitTunnelAction {
    /// Add an application to split tunnel
    Add { app: String },
    /// Remove an application from split tunnel
    Remove { app: String },
    /// List split-tunneled applications
    List,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Connect { location } => {
            let loc = location.unwrap_or_else(|| "auto".into());
            println!("Connecting to {}...", loc);
            // TODO: Send connect request to daemon
        }
        Commands::Disconnect => {
            println!("Disconnecting...");
            // TODO: Send disconnect request to daemon
        }
        Commands::Status => {
            println!("Status: Disconnected");
            // TODO: Query daemon for current status
        }
        Commands::Relays => {
            println!("Fetching relay list...");
            // TODO: Query daemon for available relays
        }
        Commands::SplitTunnel { action } => match action {
            SplitTunnelAction::Add { app } => println!("Adding {} to split tunnel", app),
            SplitTunnelAction::Remove { app } => println!("Removing {} from split tunnel", app),
            SplitTunnelAction::List => println!("No split tunnel rules configured"),
        },
        Commands::Dns { set } => {
            if let Some(server) = set {
                println!("Setting DNS to {}", server);
            } else {
                println!("DNS: System default");
            }
        }
    }

    Ok(())
}
