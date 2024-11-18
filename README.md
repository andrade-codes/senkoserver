# SenkoServer

> **Note:**
> This project is currently a proof of concept and represents the bare minimum viable product (MVP).
> It is not designed for production use, and there may be bugs, limitations, or incomplete features.
> Use at your own discretion.

## Introduction

SenkoServer is a blazing-fast static content server written in Rust, leveraging the latest web technologies for maximum performance. The server is designed with simplicity and speed in mind, providing a lightweight solution for serving static files with modern compression and protocol support.

## Features

- **High Performance**: Built with Rust for speed and efficiency.
- **Modern Protocols**: Supports HTTP/1.1 and HTTP/2, with future plans for HTTP/3 integration.
- **Static File Serving**: Serves static content directly from memory or disk.
- **Brotli Compression**: Optimized for delivering smaller file sizes to modern browsers.
- **Extensibility**: Modular design for easy customization and future enhancements.

## Getting Started

### Prerequisites

Rust (latest stable version)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/senkoserver.git
   cd senkoserver
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run
   ```

### Usage

By default, the server runs on http://127.0.0.1:8787.
Serve static files by placing them in the www/ directory (default location).

## Roadmap

### Planned Features

- Basic static file serving
- Brotli compression for optimized delivery
- HTTP/3 support (coming soon)
- In-memory file caching for ultra-fast responses
- Comprehensive logging and monitoring

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with detailed information.

# License

This project is licensed under the MIT License. See the LICENSE file for details.
