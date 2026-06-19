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
            kill(pid, SIGKILL); 
            std::cout << "[💥💥💥] Process " << pid << " has been Terminated instantly.\n";
        }
    }

    // 2. 🛡️ कंपनियों के लिए परमानेंट फायरवॉल ब्लॉकर (Dynamic Path Detection)
    void aura_execute_firewall_block(const char* target_ip) {
        if (!target_ip) return;
        std::string ip(target_ip);

        std::cout << "🛡️ [AURA SHIELD] Activating firewall block for IP: " << ip << "...\n";

        // सिस्टम से खुद iptables का सही पाTH ढूंढना
        std::string iptables_path = "/usr/sbin/iptables"; // Default Fallback
        FILE* pipe = popen("which iptables", "r");
        if (pipe) {
            char buffer[128];
            if (fgets(buffer, sizeof(buffer), pipe) != NULL) {
                iptables_path = std::string(buffer);
                iptables_path = trim_spaces(iptables_path); // एब्सोल्यूट पाथ मिला
            }
            pclose(pipe);
        }

        // सही पाथ के साथ शेल कमांड रन करना
        std::string command = iptables_path + " -A INPUT -s " + ip + " -j DROP 2>&1";
        FILE* cmd_pipe = popen(command.c_str(), "r");
        
        if (cmd_pipe) {
            char log_buffer[256];
            std::string output = "";
            while (fgets(log_buffer, sizeof(log_buffer), cmd_pipe) != NULL) {
                output += log_buffer;
            }
            int status = pclose(cmd_pipe);

            if (status == 0) {
                std::cout << "[🚀 SUCCESS] AURA SHIELD: IP " << ip << " has been blocked successfully in Netfilter!\n";
            } else {
                std::cout << "[-] FIREWALL ERROR: Execution rejected by OS.\n";
                std::cout << "🔍 Technical Detail: " << trim_spaces(output) << "\n";
                std::cout << "💡 Solution: Please run this tool with 'sudo' (Root privileges required).\n";
            }
        }
    }

    // 3. निखिल भाई का डायनामिक रूल्स लोडर (Hot-Reload Engine)
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

    // 4. 🌐 REMOTE OTA UPDATE ENGINE (बैकग्राउंड सिंक थ्रेड)
    void remote_ota_sync_worker() {
        std::string cloud_url = "https://raw.githubusercontent.com/nikhilsharma987880-bot/hybrid_log_parser/main/aura_rules.conf";
        
        while (true) {
            std::this_thread::sleep_for(std::chrono::seconds(300)); 
            std::string curl_cmd = "curl -s -o aura_rules.conf " + cloud_url;
            int status = std::system(curl_cmd.c_str());
            
            if (status == 0) {
                load_aura_rules();
                std::cout << "[🔄 AURA OTA] Rules successfully synchronized from cloud server!\n";
            }
        }
    }

    // 5. थ्रेड शुरू करने के लिए इनिशियलाइज़र
    void start_aura_ota_engine() {
        load_aura_rules();
        std::thread ota_thread(remote_ota_sync_worker);
        ota_thread.detach();
        std::cout << "[🚀 AURA CLOUD] Remote OTA Update Sync Agent Activated in Background.\n";
    }

    // 6. AI Heuristic Engine
    bool cxx_parse_line_advanced(const char* line_ptr) {
        if (!line_ptr) return false;
        std::string_view line(line_ptr);

        size_t ip_end = line.find(' ');
        if (ip_end == std::string_view::npos) return false;
        std::string ip_str(line.substr(0, ip_end));

        // LAYER 0: DYNAMIC RULES CHECK
        for (const auto& [pattern, action] : inbound_rules) {
            if (line.find(pattern) != std::string::npos) {
                std::cout << "\n🎯 [AURA DYNAMIC RULE TRIGGERED] Pattern Match: \"" << pattern 
                          << "\" -> Action Required: " << action << "\n"
                          << "[🚨 SHIELD ACTION] Threat Vector Isolated via Rules Engine!\n";
                
                // अगर एक्शन फ़ायरवॉल ट्रिगर करने का है, तो सीधे यहीं से ब्लॉक करो
                if (action.find("FIREWALL") != std::string::npos || action.find("KILL_PROCESS") != std::string::npos) {
                    aura_execute_firewall_block(ip_str.c_str());
                }
                return true; 
            }
        }

        bool is_attack = false;
        std::string attack_type = "";
        int status = 200;

        if (line.find("SELECT") != std::string_view::npos || line.find("select") != std::string_view::npos ||
            line.find("UNION") != std::string_view::npos || line.find("union") != std::string_view::npos ||
            line.find("<script>") != std::string_view::npos || line.find("%3Cscript%3E") != std::string_view::npos) {
            is_attack = true;
            attack_type = "Web Vulnerability Injection (SQLi/XSS)";
        }
        else if (line.find("etc/passwd") != std::string_view::npos || 
                 line.find(".env") != std::string_view::npos || 
                 line.find("wp-login.php") != std::string_view::npos) {
            is_attack = true;
            attack_type = "Directory Traversal / Admin Probing";
        }
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

        if (is_attack) {
            std::cout << "\n🧠 [AURA AI ALERT] " << attack_type << "\n"
                      << "[🚨 SHIELD ACTION] IP: " << ip_str << " -> संदेहास्पद गतिविधि रोकी गई!\n";
            aura_execute_firewall_block(ip_str.c_str());
            return true;
        }

        return false;
    }
}
