#include <string>
// tests whether we can instrument functions with an sret-parameter and the this-pointer-parameter
// (similar to the plain sret test)

struct large {
  long i {15};
  long x {31};
  long y {31};
  long z {31};
};

class TestClass {
  float data {0};

public:
  large test_target(std::string& str) {
    return large { .x = (int) str.length() };
  }
};


int main() {
  TestClass o;
  large s;
  std::string v("www");
  s = o.test_target(v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 6 in that case)
  if (s.x > 3) {
    return 33;
  }

  // returns 6 normally
  // returns 3 in the case v is substituted with value of v in the call above
  return o.test_target(v).x;
}