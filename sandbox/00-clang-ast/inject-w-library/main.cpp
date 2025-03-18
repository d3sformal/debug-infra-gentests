#include <iostream>
#include "lib/include/funTrace.hpp"

const char* getHello() {
    auto dumper = funTraceLib::ScopeDumper("getHello");
    
    dumper.registerReturn();
    return "world!";
}

int main() {
    auto dumper = funTraceLib::ScopeDumper("main");
    std::cout << "hello, " << getHello() << std::endl;

    dumper.registerReturn();
    return 0;
}