#include <string>
// tests whether the hooklib handles long pushes of data 
// when shared memory buffers are small (-c -s options of llcap-server)

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
  std::string v(512, 'x');
  s = o.test_target(v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 512 + 3 in that case)
  if (s.x > 512) {
    return 33;
  }

  // returns 123 (512 > 255, therefore the return code would overflow)
  return std::min(123l, o.test_target(v).x);
}