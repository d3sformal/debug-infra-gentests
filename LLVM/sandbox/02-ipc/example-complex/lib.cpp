#include "lib.h"

static int getCoeff(const std::string& coeff) {
  return coeff.size();
}

int getInt(float f){
  return f * getCoeff("test");
}
