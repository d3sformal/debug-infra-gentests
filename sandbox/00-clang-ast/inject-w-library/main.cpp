#include <iostream>
#include "lib/include/funTrace.hpp"

const char* getHello(bool fail) {
    auto INJECT_dumper = funTraceLib::ScopeDumper("getHello", 0);
    
    if (fail) {
        throw 1;
    }

    INJECT_dumper.registerReturn();
    return "world!";
}

int main() {
    auto INJECT_MAIN_ONLY_log = funTraceLib::TraceLogger("./trace.log");

    auto INJECT_dumper = funTraceLib::ScopeDumper("main", 1);
    std::cout << "hello, ";
    try {
        std::cout << getHello(true) << std::endl;
    } catch (...) {
        std::cout << "wrld" << std::endl;
    }

    getHello(false);
    getHello(false);

    INJECT_dumper.registerReturn();
    return 0;
}