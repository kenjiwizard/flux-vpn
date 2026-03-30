#!/bin/bash
# Post-installation script for macOS
# Starts the Flux VPN daemon after installation

launchctl load -w /Library/LaunchDaemons/org.fluxvpn.daemon.plist
