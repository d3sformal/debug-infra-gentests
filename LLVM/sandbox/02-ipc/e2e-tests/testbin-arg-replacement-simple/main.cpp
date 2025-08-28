#include <cassert>
#include <cstdio>
#include <cstdlib>

int test_target(int i, float f) {
  static int call_counter = 0;
  ++call_counter;

  if (call_counter == 1 && i == 0) {
    *((volatile int*)0);
  }

  if (call_counter == 4 && i > 0) {
    exit(i);
  }
  return i * f;
}


int main() {
  int result = test_target(21, 3.0f);
  test_target(3, 4.0f);
  if (result == 0) {
    *((volatile int*)0);
  }
  test_target(44, 2.0f);
  // if int_called_with_int_float is tested, one test will fail due to the check above
  test_target(0, 0);
  return result;
}