// ─── 1. अउरा मॉड्यूल रजिस्ट्रेशन (प्लग-एंड-प्ले आर्किटेक्चर) ───
#[path = "active_shield.rs"]
mod active_shield;
mod fim_chacha20;
mod ebpf_loader;
mod network_mesh;
mod telegram_alert; // टेलीग्राम अलर्ट प्लगइन भी जुड़ गया

// ─── 2. क्लीन और यूनिक सिस्टम इंपोर्ट्स ───
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// FFI: C++ Low-Level Parsing & Dynamic Rules Engine Links
unsafe extern "C" {
    fn start_aura_ota_engine(); 
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

fn main() -> io::Result<()> {
    // 1. कमांड लाइन आर्गुमेंट्स पार्स करना
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let mode = &args[1];

    // ⚡ टूल एक्टिव होते ही बैकग्राउंड क्लाउड सिंक इंजन को फायर करेगा
    unsafe {
        start_aura_ota_engine();
    }

    // ─── 3. आर्किटेक्चर राउटर (बिना कोड बदले फ्यूचर मोड्स हैंडलिंग) ───
    match mode.as_str() {
        "master" => {
            let port = args.get(2).unwrap_or(&"8080".to_string()).clone();
            println!("👑 Launching AURA Command Center Hub...");
            network_mesh::start_master_server(&port)?;
            return Ok(());
        }
        "worker" => {
            if args.len() < 4 {
                println!("❌ Usage: ./hybrid_log_parser worker <master_ip:port> <worker_id>");
                process::exit(1);
            }
            let master_addr = &args[2];
            let worker_id = &args[3];
            println!("🛠️ Launching AURA Distributed Agent Network...");
            let _mesh_stream = network_mesh::start_worker_agent(master_addr, worker_id)?;
            
            // कर्नल स्पेस में eBPF प्रोग्राम और कॉन्फ़िगरेशन लोड करना
            ebpf_loader::load_aura_ebpf_and_sync_rules("aura_rules.conf")?;
            
            thread::park(); 
            return Ok(());
        }
        "shield" => {
            if args.len() < 3 {
                println!("❌ Usage: ./hybrid_log_parser shield <log_file_path>");
                process::exit(1);
            }
            let file_path = &args[2];
            
            // लाइव शील्ड शुरू करने से पहले कर्नल eBPF को एक्टिवेट करना
            ebpf_loader::load_aura_ebpf_and_sync_rules("aura_rules.conf")?;

            if let Err(e) = active_shield::start_realtime_shield(file_path) {
                println!("❌ Active Shield Error: {:?}", e);
            }
            return Ok(());
        }
        "scan" => {
            if args.len() < 3 {
                println!("❌ Usage: ./hybrid_log_parser scan <log_file_path>");
                process::exit(1);
            }
            // यह मोड नीचे स्टैटिक एनालिसिस रन करेगा
        }
        _ => {
            println!("❌ Unknown mode: '{}'. See usage help.", mode);
            print_usage();
            process::exit(1);
        }
    }

    // ─── 4. पुराना मास्टरपीस स्टैटिक स्कैन मोड ───
    let file_path = &args[2];
    println!("🔑 Verifying system license and validity...");

    let license_file = match File::open("license.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("🛑 [ERROR] 'license.txt' not found! Please place a valid license file.");
            process::exit(1);
        }
    };
    
    let license_reader = BufReader::new(license_file); // वॉर्निंग फिक्स: 'mut' हटा दिया
    let mut lines = license_reader.lines();

    let master_key = match lines.next() {
        Some(Ok(line)) => line.trim().to_string(),
        _ => { println!("🛑 [ERROR] Invalid license file format."); process::exit(1); }
    };

    let expiry_str = match lines.next() {
        Some(Ok(line)) => line.trim().to_string(),
        _ => { println!("🛑 [ERROR] Missing expiration timestamp."); process::exit(1); }
    };

    let expiration_timestamp: u64 = expiry_str.parse().unwrap_or(0);

    if master_key != "NIKHIL-CYBER-AURA-2026" {
        println!("🛑 [ACCESS DENIED] Invalid License Key! Please contact Nikhil Sharma (Cyber Aura).");
        process::exit(1);
    }

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if current_time > expiration_timestamp {
        println!("🛑 [LICENSE EXPIRED] Your license has expired! Please contact Nikhil Sharma.");
        process::exit(1);
    }

    println!("✅ [ACCESS GRANTED] License verified. Premium multi-threaded engine activated.\n");

    // कोर ऑप्टिमाइज्ड चंक्स पार्सर इंजन
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let alert_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut chunk = Vec::new();

    for line in reader.lines() {
        let line = line?;
        chunk.push(line);

        if chunk.len() >= 10000 {
            let alert_count_clone = Arc::clone(&alert_count);
            let current_chunk = std::mem::take(&mut chunk);

            let handle = thread::spawn(move || {
                let mut local_alerts = 0;
                for item in current_chunk {
                    let c_line = CString::new(item).unwrap();
                    unsafe { if cxx_parse_line_advanced(c_line.as_ptr()) { local_alerts += 1; } }
                }
                if local_alerts > 0 { *alert_count_clone.lock().unwrap() += local_alerts; }
            });
            handles.push(handle);
        }
    }

    if !chunk.is_empty() {
        let alert_count_clone = Arc::clone(&alert_count);
        let handle = thread::spawn(move || {
            let mut local_alerts = 0;
            for item in chunk {
                let c_line = CString::new(item).unwrap();
                unsafe { if cxx_parse_line_advanced(c_line.as_ptr()) { local_alerts += 1; } }
            }
            if local_alerts > 0 { *alert_count_clone.lock().unwrap() += local_alerts; }
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }

    let final_alerts = *alert_count.lock().unwrap();
    println!("\n🎯 Hybrid Engine analysis completed. Critical threats: {}", final_alerts);

    // JSON रिपोर्ट जनरेट करना
    let report_data = serde_json::json!({
        "status": "COMPLETED",
        "developer_credit": "Nikhil Sharma (Cyber Aura)",
        "scanned_file": file_path,
        "total_threats_detected": final_alerts,
        "engine_version": "v2.0-InvincibleDistributed"
    });

    let mut report_file = File::create("threat_report.json")?;
    report_file.write_all(serde_json::to_string_pretty(&report_data).unwrap().as_bytes())?;
    println!("💾 [SUCCESS] Security report saved to 'threat_report.json'!\n");

    Ok(())
}

fn print_usage() {
    println!("❌ Invalid Command Syntax!");
    println!("💡 Available Operational Modes:");
    println!("   'scan'    -> Multi-threaded static chunk logs analyzer");
    println!("   'shield'  -> Live kernel inotify sniffer & firewall integration");
    println!("   'master'  -> AURA Distributed Command Center Panel");
    println!("   'worker'  -> AURA Agent node for multi-server synchronization");
}
