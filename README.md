# Shamir Secret Splitter

A secure, cross-platform desktop application for splitting secrets using Shamir's Secret Sharing scheme. Built with Tauri and Rust.

## What is Shamir's Secret Sharing?

Shamir's Secret Sharing is a cryptographic algorithm that divides a secret into multiple parts (shares), where a minimum threshold of shares is required to reconstruct the original secret. For example, you can split a secret into 5 shares where any 3 shares can reconstruct the secret, but 2 or fewer shares reveal nothing about the original secret.

**Use cases:**
- Securing cryptocurrency seed phrases or private keys
- Distributing master passwords among team members
- Creating secure backup schemes for sensitive data
- Multi-party authorization systems

## Features

- **Secure**: Uses the battle-tested `sharks` Rust crate implementing Shamir's Secret Sharing
- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Offline**: No network connections required - your secrets never leave your device
- **Simple**: Clean, intuitive interface for splitting and combining secrets
- **Flexible**: Configurable number of shares and threshold values

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/coffeequantz/shamir-secret-splitter/releases).

### Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or later)
- Platform-specific dependencies for Tauri (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

**Steps:**
```bash
git clone https://github.com/coffeequantz/shamir-secret-splitter.git
cd shamir-secret-splitter
npm install
npm run tauri build
```

## Usage

### Splitting a Secret

1. Enter your secret in the "Enter long secret" text area
2. Set the number of **shares** to generate (e.g., 5)
3. Set the **threshold** - minimum shares needed to reconstruct (e.g., 3)
4. Click "Split"
5. Copy and securely distribute the generated shares

**Important:** Store each share in a different secure location. The threshold determines how many shares an attacker would need to reconstruct your secret.

### Combining Shares

1. Paste the shares in the "Recombine" section (one share per line)
2. Click "Combine"
3. Your original secret will be displayed

## Security Considerations

### What This App Does

- ✅ Implements Shamir's Secret Sharing correctly using the `sharks` crate
- ✅ Runs entirely offline - no network connections
- ✅ Uses cryptographically secure random number generation
- ✅ Base64 encodes shares for easy copying/storage

### What This App Doesn't Do

- ❌ **Does not encrypt your secret** - the shares themselves contain your secret data
- ❌ **Does not protect against malware** on your computer
- ❌ **Does not validate share integrity** - corrupted shares will cause errors
- ❌ **Does not compress data** - long secrets create proportionally long shares

### Best Practices

1. **Verify the source code** before using with sensitive data (see verification section below)
2. **Use on a clean, offline computer** for maximum security
3. **Test the splitting/combining process** with non-sensitive data first
4. **Store shares in different physical locations** (different buildings, cities, etc.)
5. **Use a threshold of at least 3** (the app enforces this minimum)
6. **Keep more shares than your threshold** in case some are lost or corrupted

### Threat Model

This tool protects against:
- **Single point of failure** - losing one storage location doesn't compromise your secret
- **Insider threats** - no single person can access the secret alone
- **Partial data breaches** - attackers need multiple independent breaches

This tool does NOT protect against:
- **Compromise of your device** during splitting/combining
- **Malicious modification** of the app itself
- **Side-channel attacks** during secret processing
- **Compromise of threshold or more share storage locations**

## Verifying the Implementation

### Code Audit Checklist

To verify this implementation is secure, check:

1. **Dependencies**: 
   - `sharks = "0.5"` - Well-regarded Rust implementation of Shamir's Secret Sharing
   - `base64` - Standard encoding library
   - `tauri` - Popular framework for desktop apps

2. **Core Logic** (in `src-tauri/src/main.rs`):
   - Uses `Sharks(threshold)` correctly
   - Proper error handling for invalid parameters
   - No secret logging or storage
   - Secure conversion between shares and base64

3. **Frontend** (in `src/main.ts`):
   - Direct invocation of Rust functions
   - No network requests
   - No local storage of secrets

### Reproducing the Build

To verify the binary matches the source:

```bash
# Clone and build
git clone https://github.com/coffeequantz/shamir-secret-splitter.git
cd shamir-secret-splitter
npm install
npm run tauri build

# Compare checksums with released binaries
shasum -a 256 src-tauri/target/release/bundle/*/shamir-secret-splitter*
```

### Testing the Implementation

You can verify the implementation works correctly:

```bash
# Run the development version
npm run tauri dev

# Test with known values:
# Secret: "hello world"
# Shares: 5, Threshold: 3
# Verify that any 3 shares reconstruct the secret
# Verify that 2 shares fail to reconstruct
```

## Technical Details

### Cryptographic Properties

- **Perfect Security**: Fewer than threshold shares reveal no information about the secret
- **Galois Field**: Operations performed in GF(256) for byte-wise processing
- **Random Polynomial**: Each splitting operation uses fresh randomness
- **Information Theoretic**: Security doesn't depend on computational assumptions

### Dependencies

- **sharks**: Rust implementation of Shamir's Secret Sharing
- **base64**: RFC 4648 compliant encoding for share serialization
- **tauri**: Cross-platform desktop app framework

### Share Format

Shares are base64-encoded byte arrays containing:
- Share index (1 byte)
- Share value (same length as original secret)

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Development Setup

```bash
git clone https://github.com/coffeequantz/shamir-secret-splitter.git
cd shamir-secret-splitter
npm install
npm run tauri dev
```

## License

[MIT License](LICENSE) - Feel free to use this code for any purpose.

## Disclaimer

This software is provided as-is. While every effort has been made to implement Shamir's Secret Sharing correctly, users should verify the implementation and use appropriate security practices. The authors are not responsible for any loss of data or security breaches.

For high-value secrets, consider using this tool in combination with other security measures and on an air-gapped computer.

## Related Projects

- [sharks](https://crates.io/crates/sharks) - The underlying Rust library
- [shamirs-secret-sharing](https://github.com/dsprenkels/sss) - C implementation
- [secrets.js](https://github.com/grempe/secrets.js) - JavaScript implementation