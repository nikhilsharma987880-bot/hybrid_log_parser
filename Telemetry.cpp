#include "Telemetry.h"
#include <iostream>
#include <future>
#include <chrono>
#include <thread> // यह मिसिंग था!

void Telemetry::send_to_grid(const std::string& trigger, const std::string& action) {
    // future को एक वेरिएबल में डालो ताकि वार्निंग न आए
    auto handle = std::async(std::launch::async, [trigger, action]() {
        std::cout << "\n[📡 TELEMETRY] Syncing to Global Intelligence Grid..." << std::endl;
        std::cout << "[📡 TELEMETRY] Event: " << trigger << " | Action: " << action << std::endl;
        
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    });
}
