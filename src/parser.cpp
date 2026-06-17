#include <iostream>
#include <string_view>
#include <cstring>

extern "C" {
    
    // एडवांस्ड पार्सर फंक्शन जो IP और टाइमस्टैम्प को अलग करेगा
    bool cxx_parse_line_advanced(const char* line_ptr) {
        if (!line_ptr) return false;

        std::string_view line(line_ptr);

        // 1. IP Address निकालना (पहली स्पेस तक का डेटा)
        size_t ip_end = line.find(' ');
        if (ip_end == std::string_view::npos) return false;
        std::string_view ip = line.substr(0, ip_end);

        // 2. Status Code और ERROR चेक करना
        bool is_attack_or_error = false;
        int status = 200;

        if (line.find(" 500 ") != std::string_view::npos) {
            status = 500;
            is_attack_or_error = true;
        } else if (line.find(" 403 ") != std::string_view::npos) {
            status = 403;
            is_attack_or_error = true;
        }

        // 3. अगर एरर या संदेहास्पद एक्टिविटी है, तो पूरा डेटा कर्नल-स्टाइल फ़ास्ट प्रिंट करो
        if (is_attack_or_error) {
            size_t time_start = line.find('[');
            size_t time_end = line.find(']');
            std::string_view timestamp = "Unknown Time";
            
            if (time_start != std::string_view::npos && time_end != std::string_view::npos) {
                timestamp = line.substr(time_start + 1, time_end - time_start - 1);
            }

            // C++ का सुपरफास्ट कंसोल आउटपुट
            std::cout << "[🚨 ALERT] IP: " << ip 
                      << " | Time: " << timestamp 
                      << " | Status: " << status << " -> संदेहास्पद गतिविधि!\n";
            
            return true;
        }

        return false;
    }
}
