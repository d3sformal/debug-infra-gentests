#include <iostream>
#include "lib.h"

template<class T>
T get() {
  return T(3.14);
}

int main() {
  std::string test("testingstring");
  for (int o = 0; o < 100000; ++o) {
      int i = get<int>() + 3;
      int j = getInt(get<float>());
      auto s = strByVal(test);
      std::cout << "result: " << i << " " << j << " " << s << '\n' << o << '\n';
  }
  return 0;
}