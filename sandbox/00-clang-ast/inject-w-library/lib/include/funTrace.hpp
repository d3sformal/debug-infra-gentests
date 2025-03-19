#include "util_types.hpp"
#include <cstdlib>
#include <memory>
#include <ostream>

namespace funTraceLib {

    enum class ETraceEvent : u8;

    class TraceLogger {
    friend struct ScopeDumper;
    public:
        static TraceLogger* get();
        TraceLogger(const char* fileName);
        ~TraceLogger();
        
    private:
        void dumpTraceEvent(u64 fnId, ETraceEvent evt);
        static TraceLogger* s_logger;
        std::unique_ptr<std::ostream> m_out{nullptr};
    };

    struct ScopeDumper {
        ScopeDumper(const char* fnName, u64 fnId);
        
        void registerReturn();
        ~ScopeDumper();
        
        private:
            const char* fnName{nullptr};
            const u64 fnId;
            bool returned{false};
    };
}

