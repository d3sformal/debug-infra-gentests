#include "clang/AST/Stmt.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Rewrite/Core/Rewriter.h"
#include "clang/Tooling/CommonOptionsParser.h"
#include "clang/Tooling/Tooling.h"

#include <cassert>
#include <fstream>
#include <cstdint>
#include <map>
#include <utility>
#include <vector>



inline clang::StringRef rangeToString(const clang::SourceRange& Range, clang::SourceManager &Sm) {
    return clang::Lexer::getSourceText(clang::CharSourceRange::getTokenRange(Range), Sm, clang::LangOptions(), 0);
}

template<class ID, class KEY>
class CFunctionIdGen {
    public: 
        virtual ID getFunctionId(const KEY&) = 0;
};

template<class ID, class KEY>
class CFunctionRegistry : public CFunctionIdGen<ID, KEY> {
    public:
        virtual ID getFunctionId(const KEY& fnKey) override {
            auto found = m_mapping.find(fnKey);
            if (found != m_mapping.end()) {
              return found->second;
            }
            m_counter += 1;
            m_mapping[fnKey] = m_counter;
            return m_counter;
        }
        std::vector<std::pair<KEY, ID>> fetchFunctionIdMapping() const {
            return std::vector<std::pair<KEY, ID>>(m_mapping.cbegin(), m_mapping.cend());
        }
    private:
        ID m_counter;
        std::map<KEY, ID> m_mapping;
};

template<class T>
inline bool dumpLines(const T& iterable_outstreamable, const std::string& outputFileName) {
    std::ofstream out(outputFileName);

    if(!out) {
        return false;
    }

    for (auto &&Item : iterable_outstreamable) {
      out << Item << '\n';
    }

    return true;
}