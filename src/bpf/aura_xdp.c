use aya::Ebpf;
use aya::maps::MapData;

pub fn load_aura_ebpf_and_sync_rules(config_path: &str) -> Result<Ebpf, Box<dyn std::error::Error>> {
    // 1. फाइल लोड करो (पाथ चेक कर लेना कि aura_xdp.o कहाँ है)
    let mut bpf = Ebpf::load_file("aura_xdp.o")?;

    // 2. रस्ट को मैप्स को पहचानने के लिए मजबूर करो
    // Aya के नए वर्जन्स में, मैप्स को 'take' करना या बाइंड करना जरूरी है
    let _ring_buf_map = bpf.map_mut("aura_ringbuf")
        .map_err(|e| format!("FATAL: Map 'aura_ringbuf' not found in ELF: {:?}", e))?;
    
    let _hash_map = bpf.map_mut("blocked_ips_map")
        .map_err(|e| format!("FATAL: Map 'blocked_ips_map' not found in ELF: {:?}", e))?;

    println!("✅ [AURA eBPF] Maps successfully bound and registered.");

    // 3. (Optional) यहाँ अपना XDP प्रोग्राम अटैच करने वाला कोड रखो
    
    Ok(bpf)
}
