#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <vector>
#include <algorithm>
#include <future> // Telemetry के लिए जरूरी (Asynchronous)

// रूल्स का स्ट्रक्चर
struct RuleAction {
    std::string pattern;
    std::string action;
};

class AuraRulesEngine {
private:
    std::unordered_map<std::string, std::vector<RuleAction>> rules_map;
    std::string config_file;

    std::string trim(const std::string& str) {
        size_t first = str.find_first_not_of(" \t");
        if (std::string::npos == first) return "";
        size_t last = str.find_last_not_of(" \t");
        return str.substr(first, (last - first + 1));
    }

    // [TELEMETRY] बैकग्राउंड में ग्रिड पर डेटा भेजने का फंक्शन
    void send_to_grid(const std::string& trigger, const std::string& action) {
        std::async(std::launch::async, [trigger, action]() {
            // यहाँ असली नेटवर्क कॉल (cURL/gRPC) होगा
            std::cout << "\n[📡 TELEMETRY] Syncing event to Global Intelligence Grid..." << std::endl;
            std::cout << "[📡 TELEMETRY] Payload: {\"trigger\": \"" << trigger 
                      << "\", \"action\": \"" << action << "\"}" << std::endl;
        });
    }

public:
    AuraRulesEngine(std::string file) : config_file(file) {}

    void load_rules() {
        rules_map.clear();
        std::ifstream file(config_file);
        if (!file.is_open()) {
            std::cerr << "[!] Error opening config file: " << config_file << std::endl;
            return;
        }

        std::string line;
        while (std::getline(file, line)) {
            line = trim(line);
            if (line.empty() || line[0] == '#') continue;

            std::stringstream ss(line);
            std::string trigger, pattern, action;

            if (std::getline(ss, trigger, '|') && 
                std::getline(ss, pattern, '|') && 
                std::getline(ss, action, '|')) {
                
                RuleAction rule = { trim(pattern), trim(action) };
                rules_map[trim(trigger)].push_back(rule);
            }
        }
        std::cout << "[+] AURA: Loaded dynamic rules successfully into RAM." << std::endl;
    }

    std::string match_and_trigger(const std::string& trigger_type, const std::string& input_data) {
        if (rules_map.find(trigger_type) == rules_map.end()) return "NO_MATCH";

        for (const auto& rule : rules_map[trigger_type]) {
            if (input_data.find(rule.pattern) != std::string::npos) {
                // [TELEMETRY] मैच मिलते ही ग्रिड को रिपोर्ट करो
                send_to_grid(trigger_type, rule.action);
                return rule.action;
            }
        }
        return "NO_MATCH";
    }
};

int main() {
    AuraRulesEngine aura("aura_rules.conf");
    aura.load_rules();

    // टेस्ट केस 1
    std::string sample_log = "GET /vulnerable_endpoint?id=123 UNION SELECT null HTTP/1.1";
    std::string action = aura.match_and_trigger("INBOUND_LOG", sample_log);
    std::cout << "[Test 1] Action Triggered: " << action << std::endl;

    // टेस्ट केस 2
    std::string modified_file = "/etc/shadow";
    std::string file_action = aura.match_and_trigger("FILE_MONITOR", modified_file);
    std::cout << "[Test 2] Action Triggered: " << file_action << std::endl;

    // थ्रेड खत्म होने तक थोड़ा रुकें (सिर्फ टेस्टिंग के लिए)
    std::this_thread::sleep_for(std::chrono::milliseconds(500));

    return 0;
}
