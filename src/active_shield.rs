use notify::{Watcher, RecursiveMode, Result, Event};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc::channel;
use std::path::Path;
use std::ffi::CString;
use std::process::Command;

// FFI: C++ Low-Level AI Engine
unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

// एडवांस फ़ायरवॉल ब्लॉकिंग फ़ंक्शन (क्लीन आईपी के साथ)
fn block_ip_via_firewall(line: &str) {
    if let Some(ip) = line.split_whitespace().next() {
        // 🔥 प्रो-लेवल आईपी क्लीनअप: ब्रैकेट, कोट्स या कचरा कैरेक्टर को हटाना
        let clean_ip = ip.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != ':');
        
        println!("🛡️ [AURA SHIELD] Threat detected from IP: {}. Triggering Firewall...", clean_ip);
        
        // 🚀 एंटरप्राइज फिक्स: कंपनी सर्वर्स पर सीधे 'ufw' या 'iptables' कॉल होता है, 
        // टूल को बैकग्राउंड में प्रिविलेज दी जाती है (नीचे तरीका बताया है)
        let status = Command::new("ufw")
            .arg("deny")
            .arg("from")
            .arg(clean_ip)
            .output();

        match status {
            Ok(output) => {
                if output.status.success() {
                    println!("🚫 [BANNED] IP {} has been successfully blocked at the kernel level!", clean_ip);
                } else {
                    // अगर ufw नहीं है, तो iptables बैकअप ट्राई करेगा
                    let iptables_status = Command::new("iptables")
                        .arg("-A")
                        .arg("INPUT")
                        .arg("-s")
                        .arg(clean_ip)
                        .arg("-j")
                        .arg("DROP")
                        .output();

                    if let Ok(ip_out) = iptables_status {
                        if ip_out.status.success() {
                            println!("🚫 [BANNED via iptables] IP {} successfully blocked!", clean_ip);
                            return;
                        }
                    }
                    println!("⚠️ [WARNING] Firewall rejected the command. Privilege elevation needed.");
                }
            }
            Err(_) => println!("❌ [ERROR] Could not execute firewall binary setup."),
        }
    }
}

pub fn start_realtime_shield(file_path: &str) -> Result<()> {
    println!("🔥 AURA Active Shield: Initializing Live Kernel Sniffer...");
    println!("👀 Watching log file: [{}] for real-time cyber attacks...\n", file_path);

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive)?;

    let mut file = File::open(file_path)?;
    let mut pos = file.seek(SeekFrom::End(0))?;
    let mut reader = BufReader::new(file);

    for res in rx {
        match res {
            Ok(Event { kind, .. }) if kind.is_modify() => {
                let mut file = File::open(file_path)?;
                file.seek(SeekFrom::Start(pos))?;
                reader = BufReader::new(file);

                let mut line = String::new();
                while reader.read_line(&mut line)? > 0 {
                    let trimmed_line = line.trim();
                    if !trimmed_line.is_empty() {
                        let c_line = CString::new(trimmed_line).unwrap();
                        
                        unsafe {
                            if cxx_parse_line_advanced(c_line.as_ptr()) {
                                block_ip_via_firewall(trimmed_line);
                            }
                        }
                    }
                    line.clear();
                }
                let mut f = File::open(file_path)?;
                pos = f.seek(SeekFrom::End(0))?;
            }
            _ => {}
        }
    }
    Ok(())
}
