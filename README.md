# ⚡ Hybrid Cyber-Threat Detector Engine (v1.0-Premium)

A ultra-high-performance, kernel-style hybrid log parser built using *Rust* and *C++* via *FFI (Foreign Function Interface)*. This tool leverages Rust's fearless concurrency and multi-threading architecture alongside C++'s blazing-fast string-view manipulation to detect cyber threats in server logs within nanoseconds.
High Performance: Rust + C++ Hybrid Engine (Blazing fast, memory safe).
​AI Mutation Core: Threat detection beyond hardcoded rules.
​Kernel-Level Defense: Real-time stream parsing via inotify and direct network capabilities integration (setcap).
# Cyber Aura: Enterprise-Grade Hybrid Distributed EDR Suite (Rust + C++)

Cyber Aura is an autonomous, ultra-high-performance Endpoint Detection and Response (EDR) ecosystem engineered in a hybrid Rust and C++ architecture. Designed for modern enterprise infrastructures, it moves beyond static log parsing to deliver real-time kernel-level threat mitigation and global swarm defense.

## 🚀 Core Enterprise Features

* *Zero-Downtime Remote OTA Engine:* Asynchronous C++ background sync workers hot-reload global threat signatures (aura_rules.conf) directly into RAM without requiring system recompile or reboots.
* *Kernel-Level Executioner Hook:* Monitors operational anomalies and leverages low-level C++ FFI hooks to immediately terminate internal malicious processes via kill(pid, SIGKILL).
* *Cryptographic File Integrity Monitor (FIM):* Integrated with the Linux inotify subsystem in Rust. Any unauthorized mutation attempt on production environments triggers an instant, in-place *ChaCha20 military-grade lockdown*.
* *Pre-Log Interception via eBPF (XDP):* Injects bytecode directly into the Linux Kernel network interface layer, dropping malicious inbound packets inside the kernel before they hit user-space.
* *Distributed Mesh Network (Master-Worker):* Built on an asynchronous TCP grid in Rust. If a single edge worker agent detects a zero-day exploit, it syncs across the mesh to secure the entire fleet of servers instantly.

*Architect & Developer:* Nikhil Sharma (Cyber Aura)
## 🚀 Key Features
- *Hybrid Core:* Rust manages the multi-threaded file orchestration while C++ handles low-level parsing logic.
- *Licensing Gate:* Built-in time-bound master license verification system (30-day trial).
- *Automated Auditing:* Generates a clean, enterprise-ready threat_report.json upon completion.

## 🛠️ How to Run
To run this tool, you need a valid Master License Key.

```bash
cargo build --release
./target/release/hybrid_log_parser <log_file_path> <license_key>
