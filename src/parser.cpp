#include <iostream>
#include <string_view>
#include <cstring>
#include <algorithm>
#include <unordered_map>
#include <string>
#include <fstream>
#include <unistd.h>
#include <signal.h>
#include <thread>
#include <chrono>

// रैम में डायनामिक रूल्स स्टोर करने के लिए ग्लोबल मैप
std::unordered_map<std::string, std::string> inbound_rules;

// स्ट्रिंग ट्रिम करने के लिए हेल्पर फंक्शन
std::string trim_spaces(const std::string& str) {
    size_t first = str.find_first_not_of(" \t\r\n");
    if (std::string::npos == first) return "";
    size_t last = str.find_last_not_of(" \t\r\n");
    return str.substr(first, (last - first + 1));
}

extern "C" {
    
    // 1. कर्नल लेवल पर संदिग्ध प्रोसेस को तुरंत किल करने का फंक्शन
    void kill_malicious_process(int pid) {
        if (pid > 0) {
            std::cout << "\n[🚨 AURA KERNEL ACTION] Target PID " << pid << " identified as threat. Sending SIGKILL (-9)...\n";
            kill(pid, SIGKILL); // कर्नल लेवल किल कमांड
            std::cout << "[💥💥💥] Process " << pid << " has been Terminated instantly.\n";
        }
    }

    // 2. निखिल भाई का डायनामिक रूल्स लोडर (Hot-Reload Engine)
    void load_aura_rules() {
        inbound_rules.clear();
        std::ifstream file("aura_rules.conf");
        if (!file.is_open()) {
            std::cout << "[⚠️ AURA CONFIG] 'aura_rules.conf' not found. Operating on AI layers only.\n";
            return;
        }

        std::string line;
        while (std::getline(file, line)) {
            line = trim_spaces(line);
            if (line.empty() || line[0] == '#') continue;

            size_t first_pipe = line.find('|');
            size_t second_pipe = line.find('|', first_pipe + 1);
            
            if (first_pipe != std::string::npos && second_pipe != std::string::npos) {
                std::string trigger = trim_spaces(line.substr(0, first_pipe));
                std::string pattern = trim_spaces(line.substr(first_pipe + 1, second_pipe - first_pipe - 1));
                std::string action  = trim_spaces(line.substr(second_pipe + 1));
                
                if (trigger == "INBOUND_LOG" || trigger == "FILE_MONITOR") {
                    inbound_rules[pattern] = action;
                }
            }
        }
        std::cout << "[+] AURA: " << inbound_rules.size() << " Dynamic Rules Loaded successfully into RAM.\n";
    }

    // 3. 🌐 REMOTE OTA UPDATE ENGINE (बैकग्राउंड सिंक थ्रेड)
    void remote_ota_sync_worker() {
        std::string cloud_url = "https://raw.githubusercontent.com/nikhilsharma987880-bot/hybrid_log_parser/main/aura_rules.conf";
        
        while (true) {
            // हर 5 मिनट में बैकग्राउंड में शांत रहकर डाउनलोड करेगा
            std::this_thread::sleep_for(std::chrono::seconds(300)); 
            
            std::string curl_cmd = "curl -s -o aura_rules.conf " + cloud_url;
            int status = std::system(curl_cmd.c_str());
            
            if (status == 0) {
                // बिना इंजन को रोके लाइव रैम रीलोड (Hot-Reload)
                load_aura_rules();
                std::cout << "[🔄 AURA OTA] Rules successfully synchronized from cloud server!\n";
            }
        }
    }

    // 4. थ्रेड शुरू करने के लिए इनिशियलाइज़र फंक्शन (इसे Rust से कॉल किया गया है)
    void start_aura_ota_engine() {
        // रूल्स को पहली बार लोकल लोड करो
        load_aura_rules();
        
        // बैकग्राउंड थ्रेड को डिटैच (Detach) करके चालू कर दो
        std::thread ota_thread(remote_ota_sync_worker);
        ota_thread.detach();
        std::cout << "[🚀 AURA CLOUD] Remote OTA Update Sync Agent Activated in Background.\n";
    }

    // 5. AI Heuristic + Mutation + Dynamic Rules Parser Engine (Fully Integrated)
    bool cxx_parse_line_advanced(const char* line_ptr) {
        if (!line_ptr) return false;

        std::string_view line(line_ptr);

        // IP Address निकालना (पहली स्पेस तक का डेटा)
        size_t ip_end = line.find(' ');
        if (ip_end == std::string_view::npos) return false;
        std::string_view ip = line.substr(0, ip_end);

        // ─── LAYER 0: DYNAMIC RULES CHECK (नया एक लाइन वाला जादू) ───
        for (const auto& [pattern, action] : inbound_rules) {
            if (line.find(pattern) != std::string::npos) {
                std::cout << "\n🎯 [AURA DYNAMIC RULE TRIGGERED] Pattern Match: \"" << pattern 
                          << "\" -> Action Required: " << action << "\n"
                          << "[🚨 SHIELD ACTION] Threat Vector Isolated via Rules Engine!\n";
                return true; 
            }
        }

        bool is_attack = false;
        std::string attack_type = "";
        int status = 200;

        // ─── AI LAYER 1: SQL INJECTION & XSS DETECTOR ───
        if (line.find("SELECT") != std::string_view::npos || line.find("select") != std::string_view::npos ||
            line.find("UNION") != std::string_view::npos || line.find("union") != std::string_view::npos ||
            line.find("<script>") != std::string_view::npos || line.find("%3Cscript%3E") != std::string_view::npos) {
            is_attack = true;
            attack_type = "Web Vulnerability Injection (SQLi/XSS)";
        }

        // ─── AI LAYER 2: DIRECTORY TRAVERSAL & PROBING ───
        else if (line.find("etc/passwd") != std::string_view::npos || 
                 line.find(".env") != std::string_view::npos || 
                 line.find("wp-login.php") != std::string_view::npos) {
            is_attack = true;
            attack_type = "Directory Traversal / Admin Probing";
        }

        // ─── AI LAYER 3: CRITICAL STATUS CODES ───
        else if (line.find(" 500 ") != std::string_view::npos) {
            status = 500;
            is_attack = true;
            attack_type = "Server Critical Error Code (500)";
        } 
        else if (line.find(" 403 ") != std::string_view::npos) {
            status = 403;
            is_attack = true;
            attack_type = "Server Critical Access Denied (403)";
        }

        // ─── AI LAYER 4: MUTATION & CHARACTER ANOMALY HUNTING ───
        else {
            int anomaly_score = 0;
            for (char c : line) {
                if (c == '\'' || c == '"' || c == '`' || c == '-' || c == '\\' || c == '%') {
                    anomaly_score++;
                }
            }
            if (anomaly_score >= 5) {
                is_attack = true;
                attack_type = "Malicious Code Mutation Anomaly";
            }
        }

        // ─── OUTPUT LAYER ───
        if (is_attack) {
            size_t time_start = line.find('[');
            size_t time_end = line.find(']');
            std::string_view timestamp = "Unknown Time";
            
            if (time_start != std::string_view::npos && time_end != std::string_view::npos) {
                timestamp = line.substr(time_start + 1, time_end - time_start - 1);
            }

            std::cout << "\n🧠 [AURA AI ALERT] " << attack_type << "\n"
                      << "[🚨 SHIELD ACTION] IP: " << ip 
                      << " | Time: " << timestamp 
                      << " | Status: " << (status != 200 ? std::to_string(status) : "AI Detected") 
                      << " -> संदेहास्पद गतिविधि रोकी गई!\n";
            
            return true;
        }

        return false;
    }
}
