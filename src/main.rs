#[path = "active_shield.rs"]
mod active_shield;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// FFI: C++ Low-Level Parsing Engine
unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

fn main() -> io::Result<()> {
    // 1. Parsing Command Line Arguments
    let args: Vec<String> = env::args().collect();

    // अब टूल को चलाने के लिए <mode> और <file_path> दोनों देने होंगे
    if args.len() < 3 {
        println!("❌ Usage: ./hybrid_log_parser <mode> <log_file_path>");
        println!("💡 Mode options:");
        println!("   'scan'   -> For static multi-threaded analysis (Old Masterpiece)");
        println!("   'shield' -> For real-time kernel sniffer & firewall defense (New Plan)");
        println!("💡 Example: ./hybrid_log_parser shield server.log");
        process::exit(1);
    }

    let mode = &args[1];
    let file_path = &args[2];

    // ⚡ न्यू प्लान: अगर यूजर 'shield' मोड चुनता है, तो सीधे एक्टिव शील्ड शुरू कर दो
    if mode == "shield" {
        if let Err(e) = active_shield::start_realtime_shield(file_path) {
            println!("❌ Active Shield Error: {:?}", e);
        }
        return Ok(());
    }

    // -------------------------------------------------------------------------
    // पुराना मास्टरपीस: 'scan' मोड का पुराना कोड वैसा का वैसा ही रहेगा नीचे
    // -------------------------------------------------------------------------
    if mode != "scan" {
        println!("❌ Unknown mode: '{}'. Use 'scan' or 'shield'.", mode);
        process::exit(1);
    }

    println!("🔑 Verifying system license and validity...");

    // 2. Dynamic License Checker (Reading external license.txt)
    let license_file = match File::open("license.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("🛑 [ERROR] 'license.txt' not found! Please place a valid license file in the root directory.");
            process::exit(1);
        }
    };
    
    let mut license_reader = BufReader::new(license_file);
    let mut lines = license_reader.lines();

    // Extract License Key from Line 1
    let master_key = match lines.next() {
        Some(Ok(line)) => line.trim().to_string(),
        _ => {
            println!("🛑 [ERROR] Invalid license file format.");
            process::exit(1);
        }
    };

    // Extract Expiration Timestamp from Line 2
    let expiry_str = match lines.next() {
        Some(Ok(line)) => line.trim().to_string(),
        _ => {
            println!("🛑 [ERROR] Missing expiration timestamp in license file.");
            process::exit(1);
        }
    };

    let expiration_timestamp: u64 = match expiry_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("🛑 [ERROR] Malformed expiration timestamp.");
            process::exit(1);
        }
    };

    // 3. Security & Cryptographic License Validation
    if master_key != "NIKHIL-CYBER-AURA-2026" {
        println!("🛑 [ACCESS DENIED] Invalid License Key! Please contact Nikhil Sharma (Cyber Aura) for a valid key.");
        process::exit(1);
    }

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    if current_time > expiration_timestamp {
        println!("🛑 [LICENSE EXPIRED] Your 30-day trial has expired! Please contact Nikhil Sharma to renew your license.");
        process::exit(1);
    }

    println!("✅ [ACCESS GRANTED] License verified successfully. Premium kernel engine activated.");
    println!("⚡ Running Rust + C++ Hybrid Threat Detection Engine on: [{}]...\n", file_path);

    // 4. Core Optimized Multi-Threaded Engine (Chunking lines instead of spawning 900k threads)
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let alert_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // We will collect 10,000 lines into a chunk before spawning a thread
    let mut chunk = Vec::new();

    for line in reader.lines() {
        let line = line?;
        chunk.push(line);

        // Once the chunk reaches 10,000 lines, delegate it to a worker thread
        if chunk.len() >= 10000 {
            let alert_count_clone = Arc::clone(&alert_count);
            let current_chunk = std::mem::take(&mut chunk);

            let handle = thread::spawn(move || {
                let mut local_alerts = 0;
                for item in current_chunk {
                    let c_line = CString::new(item).unwrap();
                    unsafe {
                        if cxx_parse_line_advanced(c_line.as_ptr()) {
                            local_alerts += 1;
                        }
                    }
                }
                if local_alerts > 0 {
                    let mut num = alert_count_clone.lock().unwrap();
                    *num += local_alerts;
                }
            });
            handles.push(handle);
        }
    }

    // Process the remaining lines in the final leftover chunk
    if !chunk.is_empty() {
        let alert_count_clone = Arc::clone(&alert_count);
        let handle = thread::spawn(move || {
            let mut local_alerts = 0;
            for item in chunk {
                let c_line = CString::new(item).unwrap();
                unsafe {
                    if cxx_parse_line_advanced(c_line.as_ptr()) {
                        local_alerts += 1;
                    }
                }
            }
            if local_alerts > 0 {
                let mut num = alert_count_clone.lock().unwrap();
                *num += local_alerts;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_alerts = *alert_count.lock().unwrap();
    println!("\n🎯 Hybrid Engine analysis completed.");
    println!("❌ Total critical threats/errors detected requiring immediate action: {}", final_alerts);

    // 5. Enterprise-Grade JSON Report Generation
    println!("📝 Generating security audit report...");
    
    let report_data = serde_json::json!({
        "status": "COMPLETED",
        "developer_credit": "Nikhil Sharma (Cyber Aura)",
        "scanned_file": file_path,
        "total_threats_detected": final_alerts,
        "action_required": final_alerts > 0,
        "engine_version": "v1.2-DynamicEnterprise"
    });

    let mut report_file = File::create("threat_report.json")?;
    report_file.write_all(serde_json::to_string_pretty(&report_data).unwrap().as_bytes())?;
    
    println!("💾 [SUCCESS] Security report successfully saved to 'threat_report.json'!\n");

    Ok(())
}
