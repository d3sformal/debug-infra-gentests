#include "lib.h"

std::string strByVal(std::string s) {
  s[0] = 'a';
  return s + "alpha beta";
}
