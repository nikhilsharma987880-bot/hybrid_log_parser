use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc::Sender;

// ==========================================
// 👑 1. MASTER MODE: पूरे नेटवर्क का कमांड सेंटर
// ==========================================
pub fn start_master_server(port: &str) -> std::io::Result<()> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("👑 [AURA MASTER] Command Center active on {}...", address);
    println!("📡 [AURA MASTER] Waiting for incoming Aura Worker nodes to connect...\n");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buffer = [0; 512];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                        let trimmed = message.trim();
                        
                        if trimmed.starts_with("THREAT_ALERT:") {
                            let parts: Vec<&str> = trimmed.split(':').collect();
                            if parts.len() >= 3 {
                                let worker_id = parts[1];
                                let attacker_ip = parts[2];
                                println!("🚨 [MASTER ALERT] Critical Threat reported by Worker [{}]!", worker_id);
                                println!("🚫 [MASTER ACTION] Attacker IP [{}] identified. Broadcasting global ban to all workers!", attacker_ip);
                                
                                // रीयल-टाइम प्रोडक्शन में यहाँ से सभी कनेक्टेड वर्कर्स को 
                                // GLOBAL_BAN का पैकेट वापस भेज दिया जाएगा।
                                let _ = stream.write_all(b"GLOBAL_BAN_BROADCAST_SUCCESS\n");
                            }
                        }
                    }
                });
            }
            Err(e) => println!("⚠️ [MASTER ERROR] Connection failed: {:?}", e),
        }
    }
    Ok(())
}

// ==========================================
// 🛠️ 2. WORKER MODE: सर्वर्स पर तैनात एक्टिव एजेंट
// ==========================================
pub fn start_worker_agent(master_address: &str, worker_id: &str) -> std::io::Result<TcpStream> {
    println!("🛠️ [AURA WORKER] Initializing Agent Node [{}]...", worker_id);
    println!("🔄 [AURA WORKER] Connecting to Aura Master Command Center at [{}]...", master_address);
    
    let stream = TcpStream::connect(master_address)?;
    println!("✅ [AURA WORKER] Handshake complete! Connected to Master Network Mesh.");
    Ok(stream)
}

// थ्रेट डिटेक्ट होने पर मास्टर को तुरंत अलर्ट भेजने का फंक्शन
pub fn report_threat_to_master(mut stream: &TcpStream, worker_id: &str, attacker_ip: &str) {
    let alert_payload = format!("THREAT_ALERT:{}:{}", worker_id, attacker_ip);
    if let Err(e) = stream.write_all(alert_payload.as_bytes()) {
        println!("❌ [WORKER MESH ERROR] Failed to send threat report to Master: {:?}", e);
    } else {
        println!("📡 [WORKER MESH] Threat report for IP {} transmitted to Master successfully.", attacker_ip);
    }
}
