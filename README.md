# ⚡ Hybrid Cyber-Threat Detector Engine (v1.0-Premium)

A ultra-high-performance, kernel-style hybrid log parser built using *Rust* and *C++* via *FFI (Foreign Function Interface)*. This tool leverages Rust's fearless concurrency and multi-threading architecture alongside C++'s blazing-fast string-view manipulation to detect cyber threats in server logs within nanoseconds.
High Performance: Rust + C++ Hybrid Engine (Blazing fast, memory safe).
​AI Mutation Core: Threat detection beyond hardcoded rules.
​Kernel-Level Defense: Real-time stream parsing via inotify and direct network capabilities integration (setcap).
## 🚀 Key Features
- *Hybrid Core:* Rust manages the multi-threaded file orchestration while C++ handles low-level parsing logic.
- *Licensing Gate:* Built-in time-bound master license verification system (30-day trial).
- *Automated Auditing:* Generates a clean, enterprise-ready threat_report.json upon completion.

## 🛠️ How to Run
To run this tool, you need a valid Master License Key.

```bash
cargo build --release
./target/release/hybrid_log_parser <log_file_path> <license_key>
