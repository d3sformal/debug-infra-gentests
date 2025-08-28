#include <string>

// tests whether we can correctly instrument functions where there is sret argument, structure passed in multiple additional (IR-level) arguments

struct large {
  long i {15};
  long x {31};
  long y {31};
  long z {31};
};

struct small {
  long i {15};
  std::string* x{nullptr};
};

large test_target(small s, std::string& str) {
    return large { .x = (int) str.length() };
}

int main() {
  large l;
  small s;
  std::string v("www");
  l = test_target(s, v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 6 in that case)
  if (l.x > 3) {
    return 33;
  }

  // returns 6 normally
  // returns 3 in the case v is substituted with value of v in the call above
  return test_target(s, v).x;
}