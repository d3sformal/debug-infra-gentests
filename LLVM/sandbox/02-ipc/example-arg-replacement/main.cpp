#include <cassert>
#include <cstdio>
#include <iostream>

template <typename T> T templateTest(T x) { return x; }

int multiply_i_f(int i, float f) {
  printf("Got %d %f\n", i, f);
  printf("Returning %d\n", int(i*f));
  return i * f;
}


int main() {
  std::string v("www");

  for(int t = 0; t < 5; ++t){  
    v = templateTest<std::string>(v);
    // if templateTest is instrumented, the line below demonstrates argument replacement for
    // a C++ type
    std::cout << v << std::endl; 
    v += " x";
  }
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