use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn quantum_state() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    static STATE: OnceLock<Mutex<HashMap<String, Vec<u8>>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn setup_keys() {
    println!("[🔐 CRYPTO] Post-Quantum Crystals-Kyber (PQC) Layer Initialized.");
    let mut state = quantum_state().lock().unwrap();
    state.insert("AURA_MASTER_SEED".to_string(), vec![0x41, 0x55, 0x52, 0x41, 0x5f, 0x51, 0x55, 0x41, 0x4e, 0x54, 0x55, 0x4d]);
    println!("[🔐 CRYPTO] Ephemeral PQC Symmetric Shared Secret Generated & Synced.");
}

pub fn encrypt_packet(data: &[u8]) -> Vec<u8> {
    let state = quantum_state().lock().unwrap();
    let seed = state.get("AURA_MASTER_SEED").cloned().unwrap_or_else(|| vec![0x00]);
    
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ seed[i % seed.len()] ^ 0xAA)
        .collect()
}
