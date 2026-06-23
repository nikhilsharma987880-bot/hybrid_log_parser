#ifndef AURA_ENGINE_H
#define AURA_ENGINE_H

#include <string>
#include <vector>
#include <unordered_map>

struct RuleAction { std::string pattern; std::string action; };

class AuraRulesEngine {
private:
    std::unordered_map<std::string, std::vector<RuleAction>> rules_map;
    std::string config_file;
    std::string trim(const std::string& str);

public:
    AuraRulesEngine(std::string file);
    void load_rules();
    std::string match_and_trigger(const std::string& trigger_type, const std::string& input_data);
};

#endif
