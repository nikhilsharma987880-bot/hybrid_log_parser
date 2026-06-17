use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// FFI: हमारा खतरनाक C++ इंजन का फंक्शन
unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const std::os::raw::c_char) -> bool;
}

// हमारी सीक्रेट मास्टर लाइसेंस की 
const MASTER_LICENSE_KEY: &str = "NIKHIL-CYBER-AURA-2026";

// लाइसेंस एक्सपायरी टाइमस्टैम्प (Unix Epoch Seconds में)
// यह टाइमस्टैम्प 18 जुलाई 2026 (रात 12:00 बजे) तक वैध है। यानी ठीक 30 दिन का ट्रायल!
const EXPIRATION_TIMESTAMP: u64 = 1784419200; 

fn main() -> io::Result<()> {
    // 1. टर्मिनल से आर्गुमेंट्स (Arguments) रीड करना
    let args: Vec<String> = env::args().collect();

    // अगर यूजर ने सही से इनपुट नहीं दिया तो उसे इस्तेमाल करने का तरीका बताओ
    if args.len() < 3 {
        println!("❌ उपयोग करने का सही तरीका: cargo run -- <फाइल_का_नाम> <लाइसेंस_की>");
        println!("💡 उदाहरण: cargo run -- server.log NIKHIL-CYBER-AURA-2026");
        process::exit(1);
    }

    let file_path = &args[1];
    let user_key = &args[2];

    println!("🔑 लाइसेंस और वैलिडिटी की जांच की जा रही है...");

    // 2. लाइसेंस की (Key) वेरिफिकेशन सेटिंग
    if user_key != MASTER_LICENSE_KEY {
        println!("🛑 [ACCESS DENIED] गलत लाइसेंस की! कृपया निखिल शर्मा से संपर्क करें।");
        process::exit(1);
    }

    // 3. टाइम-बॉउंड सुरक्षा जांच (स्टैंडर्ड लाइब्रेरी का उपयोग करके)
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    if current_time > EXPIRATION_TIMESTAMP {
        println!("🛑 [LICENSE EXPIRED] 30-दिन का ट्रायल समाप्त हो चुका है! कृपया रिन्यू कराने के लिए निखिल शर्मा से संपर्क करें।");
        process::exit(1);
    }

    println!("✅ [ACCESS GRANTED] लाइसेंस स्वीकृत! प्रीमियम कर्नल इंजन एक्टिवेटेड।");
    println!("⚡ Rust + C++ हाइब्रिड साइबर-थ्रेट डिटेक्टर इंजन रन हो रहा है: [{}]...\n", file_path);

    // 4. कोर पार्सिंग इंजन (मल्टी-थ्रेडेड C++ लॉजिक)
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let alert_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for line in reader.lines() {
        let line = line?;
        let alert_count_clone = Arc::clone(&alert_count);
        let c_line = CString::new(line).unwrap();

        let handle = thread::spawn(move || {
            unsafe {
                if cxx_parse_line_advanced(c_line.as_ptr()) {
                    let mut num = alert_count_clone.lock().unwrap();
                    *num += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_alerts = *alert_count.lock().unwrap();
    println!("\n🎯 हाइब्रिड इंजन एनालिसिस समाप्त।");
    println!("❌ कुल थ्रेट्स/एरर्स जिनपर तुरंत एक्शन लेना है: {}", final_alerts);

    // 5. एडवांस्ड रिपोर्ट जनरेशन (.json फाइल में सेव करना)
    println!("📝 सुरक्षा रिपोर्ट जनरेट की जा रही है...");
    
    let report_data = serde_json::json!({
        "status": "COMPLETED",
        "developer_credit": "Nikhil Sharma (Cyber Aura)",
        "scanned_file": file_path,
        "total_threats_detected": final_alerts,
        "action_required": final_alerts > 0,
        "engine_version": "v1.0-Premium"
    });

    let mut report_file = File::create("threat_report.json")?;
    report_file.write_all(serde_json::to_string_pretty(&report_data).unwrap().as_bytes())?;
    
    println!("💾 [SUCCESS] रिपोर्ट सफलतापूर्वक 'threat_report.json' में सेव कर दी गई है!\n");

    Ok(())
}
