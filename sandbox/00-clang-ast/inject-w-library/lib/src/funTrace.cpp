#include "../include/funTrace.hpp"
#include <cassert>
#include <fstream>
#include <iostream>

enum class funTraceLib::ETraceEvent : u8 {
    ENTER_SCOPE = 0,
    LEAVE_BY_RET,
    LEAVE_BY_SCOPE
};

funTraceLib::ScopeDumper::ScopeDumper(const char* fnName, u64 fnId)
: fnName(fnName), fnId(fnId) {
    funTraceLib::TraceLogger::get()->dumpTraceEvent(fnId, funTraceLib::ETraceEvent::ENTER_SCOPE);
}

funTraceLib::ScopeDumper::~ScopeDumper() {
    if (!returned) {
        funTraceLib::TraceLogger::get()->dumpTraceEvent(fnId, funTraceLib::ETraceEvent::LEAVE_BY_SCOPE);
    }
}

void funTraceLib::ScopeDumper::registerReturn() {
    this->returned = true;
    funTraceLib::TraceLogger::get()->dumpTraceEvent(fnId, funTraceLib::ETraceEvent::LEAVE_BY_RET);
}

funTraceLib::TraceLogger* funTraceLib::TraceLogger::s_logger = nullptr;

funTraceLib::TraceLogger::TraceLogger(const char* name) {
    assert(s_logger == nullptr);
    this->m_out = std::make_unique<std::ofstream>(name);
    assert((*m_out));
    s_logger = this;
}

funTraceLib::TraceLogger* funTraceLib::TraceLogger::get() {
    assert(s_logger != nullptr);
    return s_logger;
}

funTraceLib::TraceLogger::~TraceLogger() {
    s_logger = nullptr;
    this->m_out->flush();
}

const char * etraceEvtToStr(funTraceLib::ETraceEvent e) {
    switch (e) {
        case funTraceLib::ETraceEvent::ENTER_SCOPE: return "enter scope";
        case funTraceLib::ETraceEvent::LEAVE_BY_SCOPE: return "leave scope";
        case funTraceLib::ETraceEvent::LEAVE_BY_RET: return "leave return";
    }
    return nullptr;
}

void funTraceLib::TraceLogger::dumpTraceEvent(u64 fnId, funTraceLib::ETraceEvent evt) {
    assert((*m_out));
    auto s = etraceEvtToStr(evt);
    assert(s != nullptr);
    *m_out << fnId << " " << s << '\n';
}