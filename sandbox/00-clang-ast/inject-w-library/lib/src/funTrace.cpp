#include "../include/funTrace.hpp"
#include <iostream>

void dumpEnter(const char* prefix, const char* name, u64 id) {
    std::cout << "Enter [" << prefix << "]: " << name << ' ' << id << '\n';
}

void dumpLeave(const char* prefix, const char* name, u64 id) {
    std::cout << "Leave [" << prefix << "]: " << name << ' ' << id << '\n';
}

funTraceLib::ScopeDumper::ScopeDumper(const char* fnName, u64 fnId)
: fnName(fnName), fnId(fnId) {
    dumpEnter("scope", fnName, fnId);
}

funTraceLib::ScopeDumper::ScopeDumper(const char* fnName)
: funTraceLib::ScopeDumper::ScopeDumper(fnName, getNewId()) {
}


funTraceLib::ScopeDumper::~ScopeDumper() {
    dumpLeave("scope", this->fnName, this->fnId);
}

void funTraceLib::ScopeDumper::registerReturn() {
    dumpLeave("ret", this->fnName, this->fnId);
}

std::ostream* s_outstream = nullptr;

static void funTraceLib::initializeDumpOutput(const char* filename) {
    // TODO
    if (s_outstream != nullptr) {
        *outputStream << "ERROR, ALREADY INITIALIZED";
        exit(-1);
    }
    s_outstream = outputStream;
}