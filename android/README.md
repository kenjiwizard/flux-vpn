# Flux VPN — Android

The Android client for Flux VPN, built with Kotlin.

## Prerequisites

- Android Studio Hedgehog (2023.1) or newer
- Android SDK 34+
- JDK 17+

## Building

1. Open the `android/` directory in Android Studio
2. Sync Gradle
3. Build & run on a device or emulator

## Architecture

The Android app uses the same `flux-daemon` backing service for tunnel management and security, with a dedicated Kotlin/Jetpack Compose frontend.

## License

GPL-3.0-or-later — Copyright (C) 2026 Flux VPN
