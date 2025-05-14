#include "verbosity.hpp"

bool verbose(bool Set, bool Value) {
  static bool Verbose = false;
  Verbose = Set ? Value : Verbose;
  return Verbose;
}

bool debug(bool Set, bool Value) {
  static bool Debug = false;
  Debug = Set ? Value : Debug;
  return Debug;
}