# 🕵️‍♀️ NetReconX — Intelligent Web Reconnaissance Tool
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()


**NetReconX** (formerly WebScan) is a high-performance, Rust-based tool designed for modern web reconnaissance and infrastructure analysis. It helps cybersecurity professionals, developers, and network administrators uncover hidden subdomains, detect open ports, and analyze SSL/TLS certificates — all with speed, accuracy, and reliability.

🔐 **Secure your perimeter. Know your surface. Recon with NetReconX.**

---

## ✨ Features

- 🔍 **Subdomain Scanning**: Efficiently discover subdomains via certificate transparency logs.
- 🕳️ **Open Port Detection**: Identify accessible ports across discovered hosts.
- 📜 **SSL/TLS Certificate Analysis**: Extract and inspect certificate details for each domain.
- ⚡ **High Performance**: Built with `rayon` for multi-threaded scanning (up to 256 threads).
- ⏱️ **Customizable Timeouts**: Fine-tune DNS and HTTP timeout settings.
- 🛠️ **Robust Error Handling**: Leverages `thiserror` and `anyhow` for clean, informative error reporting.

---

## ⚙️ How It Works

### 📥 Subdomain Discovery
- Queries [`crt.sh`](https://crt.sh) to fetch subdomains from public certificate transparency logs.
- Filters duplicates and malformed entries.

### 🌐 DNS Resolution
- Resolves each subdomain to verify its existence.

### 🔎 Port Scanning
- Scans a predefined list of common ports (e.g., top 100) to detect open services.

### 🔐 Certificate Analysis
- Extracts and parses SSL/TLS certificate data for metadata and security insights.

### 🧵 Thread Pool Management
- Uses a thread pool for concurrent execution, boosting performance on large scans.

---

## 📦 Installation

### 🧰 Prerequisites
- Rust (latest stable version)
- Internet access for scanning operations

### 🛠️ Installation Steps

1. **Install Rust & Cargo**
    - Use the official installer `rustup`:
   ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Verify Installation**
   ```bash
   rustc --version
   cargo --version
   ```

3. **(Optional) Update Rust**
   ```bash
   rustup update
   ```

4. **Clone the repository**
   ```bash
   git clone https://github.com/dsddevs/netreconx.git
   cd netreconx
   ```

5. **Build the project**
   ```bash
   cargo build --release
   ```

6. **Run the tool**
   ```bash
   cargo run -- <domain>
   ```

---

## 🚀 Usage

### ▶️ Basic Command
```bash
cargo run -- example.com
```

### 📤 Sample Output
- Subdomains with open ports:
  ```
  sub.example.com:80,443
  api.example.com:8080
  ```
- Output is also saved in `results/subdomains.text`.

---

## 🧱 Key Components

- 🔐 **`src/certificates`** – SSL/TLS certificate extraction and parsing.
- 🌐 **`src/clients`** – HTTP client interfaces for external API calls.
- ⚠️ **`src/errors`** – Custom error types for better diagnostics.
- 🔍 **`src/ports`** – Port scanning logic.
- 🧠 **`src/scanners`** – Core scanning engine.
- 🛰️ **`src/subdomains`** – Subdomain discovery and resolution.
- 🧵 **`src/threads`** – Thread pool and concurrency management.

---

## 📚 Dependencies

- [`reqwest`](https://docs.rs/reqwest/) – HTTP client for making requests.
- [`serde`](https://docs.rs/serde/) – Serialization/deserialization.
- [`rayon`](https://docs.rs/rayon/) – Parallelism and concurrency.
- [`trust-dns-resolver`](https://docs.rs/trust-dns-resolver/) – DNS resolution.
- [`anyhow`](https://docs.rs/anyhow/) – Error handling.
- [`thiserror`](https://docs.rs/thiserror/) – Custom error definition.
---

## 📄 License

Licensed under the [Apache-2.0 License](LICENSE)

---

## ⚠️ Disclaimer

**NetReconX** is intended for ethical and authorized use only. Always ensure you have permission to scan any domain or infrastructure. The authors are not responsible for any misuse or illegal activity involving this tool.

## 📫 Contact
telegram: @dsddevs