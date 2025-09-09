# Steggg - TCP Steganography Tool

A Rust-based steganography tool that hides encrypted messages in PNG images and transmits them over TCP connections.

## Features

- **LSB Steganography**: Embeds data using Least Significant Bit manipulation in image pixels
- **XChaCha20-Poly1305 Encryption**: Secure encryption of payloads before embedding
- **TCP Transmission**: Client-server architecture for secure message delivery
- **Password-based Key Derivation**: Uses SHA-256 for deriving encryption keys from passwords
- **Stdin Support**: Accept messages from standard input or files

## Quick Start

### Prerequisites

- Rust 1.70+ 
- A PNG cover image

### Installation

```bash
git clone <repository-url>
cd steggg
cargo build --release
```

### Usage

#### Start the Server

```bash
cargo run -- server 127.0.0.1:9000 password123
```

#### Send a Message (from file)

```bash
cargo run -- client 127.0.0.1:9000 cover.png password123 secret.txt
```

#### Send a Message (from stdin)

```bash
echo "Hello, World!" | cargo run -- client 127.0.0.1:9000 cover.png password123 -
```

## How It Works

1. **Client Side**:
   - Reads the secret message from file or stdin
   - Encrypts the message using XChaCha20-Poly1305 with password-derived key
   - Embeds the encrypted payload into the cover image using LSB steganography
   - Transmits the steganographic image to the server via TCP

2. **Server Side**:
   - Receives the steganographic image over TCP
   - Extracts the hidden payload from the image
   - Decrypts the payload using the shared password
   - Displays the original message

## Technical Details

- **Encryption**: XChaCha20-Poly1305 AEAD cipher
- **Key Derivation**: SHA-256 hash of password
- **Steganography**: LSB embedding in RGBA channels
- **Image Format**: PNG encoding/decoding
- **Network Protocol**: TCP with length-prefixed messages

## File Structure

```
src/
├── main.rs       # CLI argument parsing and main entry point
├── client.rs     # Client implementation
├── server.rs     # Server implementation  
├── crypto.rs     # Encryption/decryption functions
├── steg.rs       # Steganography embedding/extraction
└── net.rs        # TCP networking utilities
```

## Security Notes

- The steganographic image is saved as `stego.png` on the client side
- Password-based encryption provides confidentiality
- LSB modifications are visually imperceptible but detectable with analysis
- Use strong passwords for better security

## Dependencies

- `tokio` - Async runtime
- `image` - Image processing
- `chacha20poly1305` - AEAD encryption
- `sha2` - Hash functions
- `anyhow` - Error handling
- `byteorder` - Binary data serialization
- `rand` - Cryptographic randomness

## License

This project is for educational purposes. Use responsibly and in accordance with applicable laws.
