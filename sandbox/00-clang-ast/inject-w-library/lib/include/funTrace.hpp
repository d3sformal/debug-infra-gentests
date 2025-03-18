#include "util_types.hpp"
#include <cstdlib>
#include <ostream>

namespace funTraceLib {

    enum class ETraceEvent : u8 {
        ENTER_SCOPE = 0,
        LEAVE_BY_RET,
        LEAVE_BY_SCOPE
    };
    
    static void initializeDumpOutput(const char* fileName);

    struct ScopeDumper {
        ScopeDumper(const char* fnName);
        
        void registerReturn();
        
        ~ScopeDumper();
        
        private:
        
        ScopeDumper(const char* fnName, u64 fnId);
        
        static u64 getNewId() {
            static u64 id = 0;
            return id++;
        }

        const char* fnName;
        const u64 fnId;
    };
}

