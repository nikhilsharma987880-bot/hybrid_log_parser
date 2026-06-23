#ifndef TELEMETRY_H
#define TELEMETRY_H

#include <string>

// यह क्लास सिर्फ ग्रिड पर डेटा भेजने का काम करेगी
class Telemetry {
public:
    static void send_to_grid(const std::string& trigger, const std::string& action);
};

#endif
