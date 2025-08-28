#include <string>
// tests whether we can instrument a simple structure argument (the following shoudl not generate an sret)

struct small {
  int i {15};
  int x {31};
  double f {0};
};

small test_target(std::string& str) {
  return small { .x = (int) str.length() };
}

int main() {
  small s;
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