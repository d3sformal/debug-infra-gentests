#include <string>
// tests whether we can instrument functions with an sret-parameter
// sret is an LLVM attribute that is assigned to a poitner which is passed as an additional argument
// to the function (in LLVM IR) and which points to the return value
// (see https://llvm.org/docs/LangRef.html#parameter-attributes)

struct large {
  long i {15};
  long x {31};
  long y {31};
  long z {31};
};

large test_target(std::string& str) {
  return large { .x = (int) str.length() };
}

int main() {
  large s;
  std::string v("www");
  s = test_target(v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 6 in that case)
  if (s.x > 3) {
    return 33;
  }

  // returns 6 normally
  // returns 3 in the case v is substituted with value of v in the call above
  return test_target(v).x;
}