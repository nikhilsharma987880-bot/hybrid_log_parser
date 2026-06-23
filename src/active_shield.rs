use std::ffi::CString;
use std::time::Duration;
use std::os::raw::c_char;
use aya::maps::ring_buf::RingBuf;
use aya::Ebpf;

unsafe extern "C" {
    fn cxx_parse_line_advanced(line: *const c_char) -> bool;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PacketEvent {
    src_ip: u32,
    pkt_size: u32,
    payload_sample: [u8; 128],
}

pub fn start_realtime_shield(bpf_context: &mut Ebpf) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 [DIAGNOSTIC MODE] Starting AURA Shield...");

    // 1. मैप ढूँढो
    let map = bpf_context
        .maps_mut()
        .find(|(name, _)| *name == "aura_ringbuf")
        .map(|(_, m)| m)
        .ok_or("FATAL: 'aura_ringbuf' not found in BPF context")?;

    let mut ring_buf = RingBuf::try_from(map)?;
    println!("🧠 [SUCCESS] Linked to Kernel Ring Buffer. Monitoring started...");

    // 2. एक ही लूप में सब कुछ रखो
    loop {
        // अगर डेटा है तो प्रोसेस करो, नहीं तो थोड़ा इंतज़ार करो
        if let Some(event_data) = ring_buf.next() {
            println!("🚀 [AUDIT] Packet Received! Size: {} bytes", event_data.len());

            if event_data.len() >= std::mem::size_of::<PacketEvent>() {
                let event = unsafe { &*(event_data.as_ptr() as *const PacketEvent) };

                let ip_bytes = event.src_ip.to_be_bytes();
                let src_ip_str = format!("{}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
                let live_log = format!("[HACK_ATTEMPT] IP: {} Size: {}", src_ip_str, event.pkt_size);

                println!("DEBUG: Processing: {}", live_log);

                let c_line = CString::new(live_log).unwrap();
                unsafe { cxx_parse_line_advanced(c_line.as_ptr()); }
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    // Result return करने के लिए यहाँ तक पहुँचने की ज़रूरत नहीं है, लूप कभी खत्म नहीं होगा
}
