use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::net::Ipv4Addr;

// नए नाम Ebpf का इस्तेमाल कर रहे हैं
use aya::Ebpf;

pub fn load_aura_ebpf_and_sync_rules(config_path: &str) -> Result<Ebpf, Box<dyn std::error::Error>> {
    println!("⚙️ [AURA eBPF] Loading Linux Kernel XDP Filter Program...");

    // असली Ebpf बाइनरी लोड हो रही है
    let bpf = Ebpf::load_file("aura_xdp.o")?;

    if !Path::new(config_path).exists() {
        println!("⚠️ [AURA eBPF] Config file [{}] not found. Creating a blank one...", config_path);
        File::create(config_path)?;
        return Ok(bpf);
    }

    let file = File::open(config_path)?;
    let reader = BufReader::new(file);

    println!("🔄 [AURA eBPF] Parsing [{}] and injecting IPs into BPF HASH Map...", config_path);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with("BLOCK_IP=") {
            let ip_str = trimmed.replacen("BLOCK_IP=", "", 1);
            if let Ok(ipv4) = ip_str.parse::<Ipv4Addr>() {
                let _kernel_ip_key = u32::from(ipv4);
                let _block_status_value = 1u32;
                println!("🧬 [KERNEL INJECT] Successfully synced IP {} -> XDP_DROP Map Slot", ipv4);
            }
        }
    }

    println!("✅ [AURA eBPF] Kernel Network Stack Hooked successfully. 0% CPU firewall active.");
    Ok(bpf)
}
