use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use rand::RngCore;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

// मिलिट्री-ग्रेड ChaCha20 एन्क्रिप्शन फंक्शन
pub fn encrypt_file_inplace(file_path: &str, master_key_bytes: &[u8; 32]) -> std::io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Target file not found"));
    }

    // 1. ओरिजिनल फाइल का डेटा रीड करें
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 2. क्रिप्टोग्राफिक इंजन सेटअप
    let key = Key::from_slice(master_key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    
    // 12-byte का रैंडम Nonce (नंबर) जनरेट करें
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 3. डेटा को एन्क्रिप्ट करें
    let ciphertext = cipher.encrypt(nonce, buffer.as_slice())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Encryption failed"))?;

    // 4. फाइल को ओवरराइट करें (पहले Nonce लिखेंगे, फिर एन्क्रिप्टेड डेटा)
    let mut out_file = File::create(path)?;
    out_file.write_all(&nonce_bytes)?;
    out_file.write_all(&ciphertext)?;

    println!("🔒 [AURA FIM] Critical File [{}] has been locked with ChaCha20 military encryption!", file_path);
    Ok(())
}
