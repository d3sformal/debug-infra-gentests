#include <cassert>
#include <cstdio>
#include <cstdlib>
#include <iostream>

void inner(int num) {
  if (num == 0) {
    throw "exception";
  }
}

struct Destroy {
  ~Destroy() {
    std::cout << "Dtor\n";
  }
};

int multiply_i_f(int i, float f) {
  // force cleanup code to be generated
  Destroy d;
  static int call_counter = 0;
  if (call_counter == 0) {
    ++call_counter;
    return 0;
  }
  // can throw an exception
  inner(i);
  
  return i * f;
}


int main() {
  multiply_i_f(11, 12);
  int result = multiply_i_f(21, 3.0f);

  multiply_i_f(3, 4.0f);
  if (result == 0) {
    *((volatile int*)0);
  }

  try {
    multiply_i_f(44, 2.0f);
  } catch (const char*) {
    return 0;
  }

  multiply_i_f(0, 0);
  return result;
}