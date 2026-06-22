use crate::{ai_predictor, quantum_crypto, grid_intelligence, hardware_xdp};

pub fn load_all_advanced_modules() {
    println!("[+] Initializing Aura Intelligence Core...");
    
    // AI को एक्टिवेट करो
    ai_predictor::init();
    
    // क्वांटम क्रिप्टोग्राफी लेयर सेट करो
    quantum_crypto::setup_keys();
    
    // ग्रिड को स्कैनिंग के लिए तैयार करो
    grid_intelligence::map_nodes();
    
    // XDP को हार्डवेयर लेवल पर हुक करो
    hardware_xdp::attach_to_nic();

    println!("[+] All Aura Advanced Modules: Active & Optimized.");
} // <--- यह ब्रैकेट बंद होना छूट गया था, अब फिक्स है!
