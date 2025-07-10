#include <cassert>
#include <cstdio>
#include <cstdlib>

int multiply_i_f(int i, float f) {
  static int call_conter = 0;
  ++call_conter;

  if (call_conter == 1 && i == 0) {
    *((volatile int*)0);
  }

  if (call_conter == 4 && i > 0) {
    exit(i);
  }
  return i * f;
}


int main() {
  int result = multiply_i_f(21, 3.0f);
  multiply_i_f(3, 4.0f);
  if (result == 0) {
    *((volatile int*)0);
  }
  multiply_i_f(44, 2.0f);
  // if int_called_with_int_float is tested, one test will fail due to the check above
  multiply_i_f(0, 0);
  return result;
}