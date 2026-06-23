#include "AuraEngine.h"
#include "Telemetry.h"
#include <fstream>
#include <sstream>
#include <iostream>

AuraRulesEngine::AuraRulesEngine(std::string file) : config_file(file) {}

std::string AuraRulesEngine::trim(const std::string& str) {
    size_t first = str.find_first_not_of(" \t");
    if (std::string::npos == first) return "";
    size_t last = str.find_last_not_of(" \t");
    return str.substr(first, (last - first + 1));
}

void AuraRulesEngine::load_rules() {
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

std::string AuraRulesEngine::match_and_trigger(const std::string& trigger_type, const std::string& input_data) {
    if (rules_map.find(trigger_type) == rules_map.end()) return "NO_MATCH";

    for (const auto& rule : rules_map[trigger_type]) {
        if (input_data.find(rule.pattern) != std::string::npos) {
            Telemetry::send_to_grid(trigger_type, rule.action);
            return rule.action;
        }
    }
    return "NO_MATCH";
}
