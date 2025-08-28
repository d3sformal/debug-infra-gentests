#include <string>
// tests whether we can instrument functions with the this pointer

class CX {
public:
  int test_target(std::string& s){ return (int) s.length();};
};

int main() {
  CX klass;
  std::string v("www");
  int rv = klass.test_target(v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 6 in that case)
  if (rv > 3) {
    return 33;
  }

  // returns 6 normally
  // returns 3 in the case v is substituted with value of v in the call above
  return klass.test_target(v);
}