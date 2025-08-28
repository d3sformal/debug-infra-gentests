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

int test_target(int i, float f) {
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
  test_target(11, 12);
  int result = test_target(21, 3.0f);

  test_target(3, 4.0f);
  if (result == 0) {
    *((volatile int*)0);
  }

  try {
    test_target(44, 2.0f);
  } catch (const char*) {
    return 0;
  }

  test_target(0, 0);
  return result;
}