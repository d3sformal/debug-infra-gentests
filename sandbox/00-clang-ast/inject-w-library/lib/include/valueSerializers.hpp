#ifndef VSTR_VAL_SERIALIZERS_DUMP_HPP
#define VSTR_VAL_SERIALIZERS_DUMP_HPP
#include <array>
#include <cassert>
#include <cstddef>
#include <cstring>
#include <type_traits>
#include <vector>

namespace funTraceLib::dump::serializers {
    using BUFF_DATA_T = std::byte;
    using BUFF_T = std::vector<BUFF_DATA_T>;
    using TID_T = std::byte;
    #define TYPEID_SZ sizeof(TID_T)

    template<typename T, TID_T ID,
    std::enable_if_t<(std::is_integral_v<T> || std::is_floating_point_v<T>), bool> = true>
    struct DefaulSerializationTrait {
        using Self = DefaulSerializationTrait<T, ID>;
        static constexpr TID_T typeId{ID}; 
        static constexpr std::size_t getStandaloneSize() {
            return sizeof(T);
        }

        static constexpr std::size_t getSerializedSize() {
            return TYPEID_SZ + getStandaloneSize();
        }

        
        static void serializeInto(BUFF_T& target, std::size_t index, const T& value) {
            auto requiredSize = index + Self::getSerializedSize();
            if (target.size() < requiredSize) {
                target.resize(requiredSize);
            }
            
            constexpr auto sz { Self::getSerializedSize() };
            std::array<BUFF_DATA_T, sz> buff;
            std::memcpy(&buff, &value, getStandaloneSize());
            
            std::memcpy(target.data() + index + TYPEID_SZ, &buff, getStandaloneSize());
            target[index] = typeId;
        }
    };

    namespace type_ids {

        // iterates (type-value) pairs in the included file
        // and creates a function templates that are
        // toggled via the the corresponding types, returning the
        // correspodning values
        // AKA the X macro
        // https://en.wikipedia.org/wiki/X_macro
        #define VSTR_VAL_SPAIR(ty, va) template<class T, std::enable_if_t<std::is_same_v<ty, T>, bool> = true> constexpr TID_T resolveTypeId() { return std::byte(va); }
        #include "typeMap.hpp"
        #undef VSTR_VAL_SPAIR
    }

    template<class T>
    struct Serializer {
        using STrait = DefaulSerializationTrait<T, type_ids::resolveTypeId<T>()>;
        static BUFF_T serialize(const T& value) {
            BUFF_T rv;
            STrait::serializeInto(rv, 0, value);
            return rv;
        }
    };
}

#endif // VSTR_VAL_SERIALIZERS_DUMP_HPP