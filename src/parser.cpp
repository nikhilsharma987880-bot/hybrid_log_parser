#include <iostream>
#include <string_view>
#include <cstring>
#include <algorithm>

extern "C" {
    
    // AI Heuristic, Mutation & Status Code Parser Engine (Fully Mixed)
    bool cxx_parse_line_advanced(const char* line_ptr) {
        if (!line_ptr) return false;

        std::string_view line(line_ptr);

        // 1. IP Address निकालना (पहली स्पेस तक का डेटा)
        size_t ip_end = line.find(' ');
        if (ip_end == std::string_view::npos) return false;
        std::string_view ip = line.substr(0, ip_end);

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

        // ─── AI LAYER 3: CRITICAL STATUS CODES (पुराना ढांचा) ───
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
            // अगर एक ही लाइन में 5 से ज्यादा म्यूटेशन कैरेक्टर्स हैं
            if (anomaly_score >= 5) {
                is_attack = true;
                attack_type = "Malicious Code Mutation Anomaly";
            }
        }

        // ─── OUTPUT LAYER: अगर थ्रेट मिला तो लाइव कर्नल अलर्ट फ्लैश करो ───
        if (is_attack) {
            size_t time_start = line.find('[');
            size_t time_end = line.find(']');
            std::string_view timestamp = "Unknown Time";
            
            if (time_start != std::string_view::npos && time_end != std::string_view::npos) {
                timestamp = line.substr(time_start + 1, time_end - time_start - 1);
            }

            // सुपरफास्ट कंसोल अलर्ट आउटपुट
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
