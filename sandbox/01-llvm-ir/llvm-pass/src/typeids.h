#ifndef LLCAP_TYPEIDS
#define LLCAP_TYPEIDS

// For argument capture we only require the knowledge of argument size.
//
// This is due to the separation of serialization and deserialization from the
// storage of the data - only the hooklib serializes and deserializes the data
// and therefore llcap-server can remain oblivious to the exact meaning of the
// types
//
// Llcap-server has to, however know how to differentiate between a custom type
// and a primitive type to support variable-length objects.
//
// The options are:
// 1) hard-code the parsing logic (handle each and every value of the enum)
// 2) create a self-contained library with a flexible iterface that
//    takes a size ID and returns either
//    A) how many bytes to read EXACTLY
//    B) how many bytes to read FORWARD and call the library again, with a
//    pointer to the received data C) error
//
//    Option B is necessary if we really want to abstract parsing from
//    llcap-server in the case of dynamically-sized types (e.g. if string
//    serialization lays out data as such: CAPACITY (4B) | SIZE (4B) | DATA
//    (SIZE-B), we would first require to (B)) read 8 bytes and require to be
//    called again with the read data, read the last 4B and return this number
//    as an exact (A)) answer)
// For now, option 1) is more straightforward as the only extra supported type
// is the string. This file is expected to be imported by:
//    1. LLVM Pass for function identifier maps generation and hook call
//    instrumentation
//    2. llcap-server for function identifier maps parsing (converts to either
//    size or a custom
//       size handler for custom/dynamic types)

static_assert(sizeof(unsigned short) == 2, "Unexpected size of unsigned short");

enum class LlcapSizeType : unsigned short {
  LLSZ_INVALID = 0U,
  LLSZ_8 = 1U,  // read 1 byte
  LLSZ_16 = 2U, // read 2 bytes
  LLSZ_24 = 3U,
  LLSZ_32 = 4U,
  LLSZ_40 = 5U,
  LLSZ_48 = 6U,
  LLSZ_56 = 7U,
  LLSZ_64 = 8U,
  LLSZ_72 = 9U,
  LLSZ_80 = 10U,
  LLSZ_88 = 11U,
  LLSZ_96 = 12U,
  LLSZ_104 = 13U,
  LLSZ_112 = 14U,
  LLSZ_120 = 15U,
  LLSZ_128 = 16U, // read 16 bytes
  // mind the gap!
  // 17 - 1024 to allow for longer primitive types if needed

  // no primitive types beyond this line:
  LLSZ_FLAT_MAX_EXCL = 1025U, // will be interpreted as an invalid size
  LLSZ_CSTR = 1026U,          // read a C string (until a zero byte is reached)
  LLSZ_CUSTOM = 1027U,  // a payload of (LEN | DATA) where LEN = length of the
                        // entire payload (including LEN, which itself is 8B)
  LLSZ_MAX_EXCL = 1028U // will be interpreted as an invalid size
};

inline bool isValid(LlcapSizeType T) {
  return T != LlcapSizeType::LLSZ_INVALID &&
         T != LlcapSizeType::LLSZ_FLAT_MAX_EXCL &&
         T != LlcapSizeType::LLSZ_MAX_EXCL;
}

#endif // LLCAP_TYPEIDS