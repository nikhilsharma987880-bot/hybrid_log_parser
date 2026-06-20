use notify::{Watcher, RecursiveMode, Result, Event};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc::channel;
use std::path::Path;
use std::ffi::CString;

// FFI: C++ Low-Level AI Engine Links (AURA Kernel Core)
unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

pub fn start_realtime_shield(file_path: &str) -> Result<()> {
    println!("🔥 AURA Active Shield: Initializing Live Kernel Sniffer...");
    println!("👀 Watching log file: [{}] for real-time cyber attacks...\n", file_path);

    // इनोटिफाई (FS Event) चैनल सेटअप
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive)?;

    // फाइल के अंत (EOF) में पोजीशन सेट करना ताकि पुराना डेटा दोबारा प्रोसेस न हो
    let mut file = File::open(file_path).map_err(|e| notify::Error::io(e))?;
    let mut pos = file.seek(SeekFrom::End(0)).map_err(|e| notify::Error::io(e))?;
    let mut reader = BufReader::new(file);

    // जैसे ही फाइल में कोई नया बदलाव (Modify) होगा, यह लूप ट्रिगर होगा
    for res in rx {
        match res {
            Ok(Event { kind, .. }) if kind.is_modify() => {
                let mut file = File::open(file_path).map_err(|e| notify::Error::io(e))?;
                file.seek(SeekFrom::Start(pos)).map_err(|e| notify::Error::io(e))?;
                reader = BufReader::new(file);

                let mut line = String::new();
                while reader.read_line(&mut line).map_err(|e| notify::Error::io(e))? > 0 {
                    let trimmed_line = line.trim();
                    if !trimmed_line.is_empty() {
                        
                        // ─── नया एडवांस सेफ्टी चेक ───
                        // अगर लाइन FILE_MONITOR से शुरू हो रही है, तो यह केवल लोकल फाइल सिस्टम अलर्ट है
                        if trimmed_line.starts_with("FILE_MONITOR") {
                            println!("\n📂 [AURA MONITOR] System file event detected. Monitoring Integrity...");
                            // सीधे C++ इंजन को पास करो, पर कोई भी लोकल डुप्लिकेट फायरवॉल एक्शन यहाँ से मत लो
                            let c_line = CString::new(trimmed_line).unwrap();
                            unsafe {
                                cxx_parse_line_advanced(c_line.as_ptr());
                            }
                            line.clear();
                            continue;
                        }

                        // नॉर्मल लॉग लाइन्स (IP वाली) को सीधे C++ कोर हीूरिस्टिक लेयर पर भेजें
                        let c_line = CString::new(trimmed_line).unwrap();
                        unsafe {
                            // C++ फ़ंक्शन खुद नेटफ़िल्टर/iptables को रूट प्रिविलेज के साथ हैंडल कर लेगा
                            cxx_parse_line_advanced(c_line.as_ptr());
                        }
                    }
                    line.clear();
                }
                
                // नया पोजीशन सेव करें ताकि अगले इवेंट पर सिर्फ नया डेटा रीड हो
                let mut f = File::open(file_path).map_err(|e| notify::Error::io(e))?;
                pos = f.seek(SeekFrom::End(0)).map_err(|e| notify::Error::io(e))?;
            }
            _ => {}
        }
    }
    Ok(())
}
