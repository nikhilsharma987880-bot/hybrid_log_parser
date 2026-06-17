fn main() {
    // यह Rust को बताता है कि अगर parser.cpp में कोई बदलाव हो, तो इसे दोबारा कंपाइल करो
    println!("cargo:rerun-if-changed=src/parser.cpp");

    // C++ कंपाइलर इंजन
    cc::Build::new()
        .cpp(true)
        .file("src/parser.cpp")
        .compile("logparser"); // यह 'liblogparser.a' नाम की बाइनरी बनाएगा जिसे लिंकर ढूंढेगा
}
