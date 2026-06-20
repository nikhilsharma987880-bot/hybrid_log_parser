#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <vector>
#include <algorithm>

// रूल्स का स्ट्रक्चर
struct RuleAction {
    std::string pattern;
    std::string action;
};

class AuraRulesEngine {
private:
    // RAM Storage: Trigger -> List of (Pattern, Action)
    std::unordered_map<std::string, std::vector<RuleAction>> rules_map;
    std::string config_file;

    // स्ट्रिंग को ट्रिम करने के लिए हेल्पर फंक्शन
    std::string trim(const std::string& str) {
        size_t first = str.find_first_not_of(" \t");
        if (std::string::npos == first) return "";
        size_t last = str.find_last_not_of(" \t");
        return str.substr(first, (last - first + 1));
    }

public:
    AuraRulesEngine(std::string file) : config_file(file) {}

    // फाइल को लाइव रैम में लोड करने का फंक्शन (Hot-Reload)
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
            if (line.empty() || line[0] == '#') continue; // कमेंट्स और खाली लाइन छोड़ो

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

    // नैनो-सेकंड्स मैचिंग इंजन
    std::string match_and_trigger(const std::string& trigger_type, const std::string& input_data) {
        if (rules_map.find(trigger_type) == rules_map.end()) return "NO_MATCH";

        // रैम के हैश-मैप में लूप चलाकर पैटर्न ढूंढना
        for (const auto& rule : rules_map[trigger_type]) {
            if (input_data.find(rule.pattern) != std::string::npos) {
                return rule.action; // मैच होते ही एक्शन रिटर्न करो
            }
        }
        return "NO_MATCH";
    }
};

int main() {
    AuraRulesEngine aura("aura_rules.conf");
    
    // 1. रूल्स लोड करो
    aura.load_rules();

    // 2. टेस्ट केस - इनबाउंड अटैक आया (जैसे पुराना फ़ायरवॉल काम करता था)
    std::string sample_log = "GET /vulnerable_endpoint?id=123 UNION SELECT null HTTP/1.1";
    std::string action = aura.match_and_trigger("INBOUND_LOG", sample_log);
    std::cout << "\n[Test 1] Log Match Result: " << action << std::endl; // Output: BLOCK_IP_UFW

    // 3. टेस्ट केस - नया फाइल प्रोटेक्शन अलर्ट आया
    std::string modified_file = "/etc/shadow";
    std::string file_action = aura.match_and_trigger("FILE_MONITOR", modified_file);
    std::cout << "[Test 2] File Match Result: " << file_action << std::endl; // Output: KILL_PROCESS_PID

    return 0;
}
