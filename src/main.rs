mod active_shield;
mod fim_chacha20;
mod ebpf_loader;
mod network_mesh;
mod aura_plugins;
mod ai_predictor;
mod quantum_crypto;
mod grid_intelligence;
mod hardware_xdp;
mod telegram_alert;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

unsafe extern "C" {
    fn start_aura_ota_engine();
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

fn verify_license() {
    let license_file = File::open("license.txt").unwrap_or_else(|_| {
        eprintln!("🛑 [ERROR] 'license.txt' missing!");
        process::exit(1);
    });
    let mut lines = BufReader::new(license_file).lines();
    let master_key = lines.next().and_then(|l| l.ok()).unwrap_or_default();
    let expiry_str = lines.next().and_then(|l| l.ok()).unwrap_or_default();
    
    if master_key.trim() != "NIKHIL-CYBER-AURA-2026" {
        eprintln!("🛑 [ACCESS DENIED] Invalid Key.");
        process::exit(1);
    }
    let expiration_timestamp: u64 = expiry_str.trim().parse().unwrap_or(0);
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if current_time > expiration_timestamp {
        eprintln!("🛑 [LICENSE EXPIRED]");
        process::exit(1);
    }
}

fn main() -> io::Result<()> {
    verify_license();
    aura_plugins::load_all_advanced_modules();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { process::exit(1); }
    unsafe { start_aura_ota_engine(); }

    match args[1].as_str() {
        "master" => network_mesh::start_master_server(&args.get(2).unwrap_or(&"8080".to_string()))?,
        "worker" => {
            network_mesh::start_worker_agent(&args[2], &args[3])?;
            ebpf_loader::load_aura_ebpf_and_sync_rules("aura_rules.conf")?;
            thread::park();
        }
        "shield" => {
            ebpf_loader::load_aura_ebpf_and_sync_rules("aura_rules.conf")?;
            // यहाँ फिक्स है: '?' हटा दिया है, अब यह बिल्ड होगा!
            if let Err(e) = active_shield::start_realtime_shield(&args[2]) {
                eprintln!("❌ Shield Error: {:?}", e);
            }
        }
        "scan" => {
            let reader = BufReader::new(File::open(&args[2])?);
            let alert_count = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            let mut chunk = Vec::new();
            for line in reader.lines() {
                chunk.push(line?);
                if chunk.len() >= 10000 {
                    let alert_count_clone = Arc::clone(&alert_count);
                    let current_chunk = std::mem::take(&mut chunk);
                    handles.push(thread::spawn(move || {
                        for item in current_chunk {
                            let c_line = CString::new(item).unwrap();
                            unsafe { if cxx_parse_line_advanced(c_line.as_ptr()) { *alert_count_clone.lock().unwrap() += 1; } }
                        }
                    }));
                }
            }
            for h in handles { h.join().unwrap(); }
            println!("\n🎯 Threats: {}", *alert_count.lock().unwrap());
        }
        _ => (),
    }
    Ok(())
}
