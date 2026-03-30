# Contributing to Flux VPN

Thank you for your interest in contributing to Flux VPN! Every contribution matters, whether it's fixing a typo, improving documentation, reporting a bug, or building an entirely new feature.

## Getting Started

1. **Fork** the repository on [GitHub](https://github.com/kenjiwizard/flux-vpn)
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/flux-vpn.git
   cd flux-vpn
   git submodule update --init
   ```
3. **Create a branch** for your work:
   ```bash
   git checkout -b feature/my-awesome-change
   ```
4. **Make your changes**, write tests if applicable, and commit.
5. **Push** to your fork and open a **Pull Request**.

## Development Prerequisites

- **Rust** 1.80+ (see `rust-toolchain.toml`)
- **Node.js** 20+ and npm (for the desktop GUI)
- **Android Studio** (for Android development)
- **Xcode** 15+ (for iOS development)

## Code Style

- **Rust**: Run `cargo fmt` and `cargo clippy` before submitting.
- **TypeScript/JavaScript**: Run `npm run lint` in `desktop/packages/flux-vpn/`.
- **Kotlin**: Follow standard Android/Kotlin conventions.
- **Swift**: Follow standard Swift style guidelines.

## Commit Messages

Use clear, descriptive commit messages. Prefix with the area of change:

```
flux-daemon: Fix tunnel reconnection on network change
desktop: Update tray icon for dark mode
docs: Add build instructions for ARM64
```

## Reporting Bugs

Open an issue on [GitHub](https://github.com/kenjiwizard/flux-vpn/issues) with:

- Your OS and version
- Steps to reproduce
- Expected vs. actual behavior
- Relevant logs (use `flux-problem-report` to generate a report)

## Security Issues

**Do not** open a public issue for security vulnerabilities. See [SECURITY.md](SECURITY.md) for responsible disclosure instructions.

## Community

- Follow [@flux_vpn](https://x.com/flux_vpn) on 𝕏 for updates
- Follow the creator [@wizard_0x](https://x.com/wizard_0x) (Kenji)

## License

By contributing, you agree that your contributions will be licensed under the [GPL-3.0 License](LICENSE.md).
