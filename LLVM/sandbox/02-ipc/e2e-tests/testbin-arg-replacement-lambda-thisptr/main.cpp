#include <string>
// tests whether we can instrument lambdas
// follows the thisptr test case as lambdas should create structs with operator()

int main() {
  auto lambda = [](std::string& s) {return s.length();};
  std::string v("www");
  int rv = lambda(v);
  v += "123";

  // returns 33 only if the above argument was substituted with the 
  // argument value of the call below (s.x is 6 in that case)
  if (rv > 3) {
    return 33;
  }

  // returns 6 normally
  // returns 3 in the case v is substituted with value of v in the call above
  return lambda(v);
}