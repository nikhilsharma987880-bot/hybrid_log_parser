#include "AuraEngine.h"
#include "Telemetry.h"

int main() {
    AuraRulesEngine aura("aura_rules.conf");
    aura.load_rules();
    // यहाँ से कॉल करोगे
    return 0;
}
