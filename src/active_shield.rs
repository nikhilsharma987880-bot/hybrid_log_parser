use notify::{Watcher, RecursiveMode, Result, Event};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc::channel;
use std::path::Path;
use std::ffi::CString;

// अन्य सभी प्लगइन्स को बिना main.rs को डिस्टर्ब किए यहीं हुक किया
use crate::fim_chacha20;
use crate::ebpf_loader;
use crate::telegram_alert; // 👈 टेलीग्राम मॉड्यूल हुक हो गया

// FFI: C++ Low-Level AI Engine Links (AURA Kernel Core)
unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

// ─── डायनेमिक कॉन्फ़िगरेशन रूल्स पार्सर और एक्शन इंजन ───
pub fn process_active_rule(log_line: &str) {
    // 1. ChaCha20 File Integrity Monitor (FIM) ट्रिगर
    if log_line.contains("FILE_MONITOR") && log_line.contains("/var/www/html/index.php") {
        println!("🚨 [AURA FIM DETECTED] Unauthorized modification attempt on production index.php!");
        
        let master_key = [0u8; 32]; 
        if let Err(e) = fim_chacha20::encrypt_file_inplace("/var/www/html/index.php", &master_key) {
            println!("❌ [AURA FIM ERROR] Failed to lock file: {:?}", e);
        }

        // 🔥 नया फ़ीचर अलर्ट: सीधे टेलीग्राम पर मैसेज फेंकना बिना कोर डिस्टर्ब किए!
        telegram_alert::send_telegram_alert("⚠️ ATTACK ALERT: /var/www/html/index.php was altered! ChaCha20 lockdown initiated.");
    }
    
    // 2. ऑटो-सिंक कर्नल लेयर: अगर कोई इनबाउंड अटैक डिटेक्ट होता है
    if log_line.contains("BLOCK_IP_UFW") || log_line.contains("HACK_ATTEMPT") {
        println!("🧬 [AURA eBPF SYNC] Attack vector detected in logs. Synchronizing Kernel XDP Maps with aura_rules.conf...");
        if let Err(e) = ebpf_loader::load_aura_ebpf_and_sync_rules("aura_rules.conf") {
            println!("❌ [AURA eBPF ERROR] Kernel sync failed: {:?}", e);
        }
        
        // टेलीग्राम अलर्ट फॉर हैकिंग एटेम्पट
        telegram_alert::send_telegram_alert("🔥 KERNEL BLOCK: Intrusion pattern matched. Syncing eBPF XDP Firewall Drop Maps.");
    }
}

pub fn start_realtime_shield(file_path: &str) -> Result<()> {
    println!("🔥 AURA Active Shield: Initializing Live Kernel Sniffer...");
    println!("👀 Watching log file: [{}] for real-time cyber attacks...\n", file_path);

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive)?;

    let mut file = File::open(file_path).map_err(|e| notify::Error::io(e))?;
    let mut pos = file.seek(SeekFrom::End(0)).map_err(|e| notify::Error::io(e))?;

    // जैसे ही फाइल में कोई नया बदलाव (Modify) होगा, यह लूप ट्रिगर होगा
    for res in rx {
        match res {
            Ok(Event { kind, .. }) if kind.is_modify() => {
                let mut file = File::open(file_path).map_err(|e| notify::Error::io(e))?;
                file.seek(SeekFrom::Start(pos)).map_err(|e| notify::Error::io(e))?;
                let mut reader = BufReader::new(file); // वॉर्निंग फिक्स: सीधे यहीं डिफाइन किया

                let mut line = String::new();
                while reader.read_line(&mut line).map_err(|e| notify::Error::io(e))? > 0 {
                    let trimmed_line = line.trim();
                    if !trimmed_line.is_empty() {
                        
                        // डायनेमिक रूल्स प्लगइन चेकर चलाएं
                        process_active_rule(trimmed_line);

                        if trimmed_line.starts_with("FILE_MONITOR") {
                            println!("\n📂 [AURA MONITOR] System file event detected. Monitoring Integrity...");
                            let c_line = CString::new(trimmed_line).unwrap();
                            unsafe { cxx_parse_line_advanced(c_line.as_ptr()); }
                            line.clear();
                            continue;
                        }

                        let c_line = CString::new(trimmed_line).unwrap();
                        unsafe { cxx_parse_line_advanced(c_line.as_ptr()); }
                    }
                    line.clear();
                }
                
                let mut f = File::open(file_path).map_err(|e| notify::Error::io(e))?;
                pos = f.seek(SeekFrom::End(0)).map_err(|e| notify::Error::io(e))?;
            }
            _ => {}
        }
    }
    Ok(())
}
