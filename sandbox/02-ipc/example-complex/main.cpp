#include <cassert>
#include <cstdint>
#include <cstdio>
#include <iostream>
#include <limits>
#include <vector>
#include "./lib.h"

namespace foo_namespace {
namespace bar_namespace {
void foo(int, float) {
  // just ReturnStmt (no children!)
  return;
}
} // namespace bar_namespace

void baz(int i) { assert(i > 0); }

} // namespace foo_namespace

template <class T> auto addAuto(T a, T b) { return a + b; }

float x = 1;
float &retRef() { return x; }

int int_called_with_int_float(int i, float f) {
  // Var (x, builtintype::int) -> ImplicitCast
  int x = i * f;

  // Var (y, auto) -> -> BinaryOperator
  auto y = i * f;
  // ReturnStmt -> ImplicitCast -> BinaryOperator
  return i * f;
  // ImplicitCast -> BinaryOperator
  return y;

  // =>
  // return (stmt);

  // should be transformable to

  // auto x = (stmt);
  // ... instrument fn return ...
  // return x;

  // If -> Compound -> Return
  if (true) {
    return x;
  }

  // If -> Return
  if (false)
    return y;
  if (false)
    return y;

  // NOT part of compound expression?
  // replace Return with Compound, insert Return into Compound
  // replace

  return retRef();
}

float float_called_with_double_int([[maybe_unused]] double d, int i) {
  return d * i;
}

int everything(int) {
  return int_called_with_int_float(0, 3.2f) +
         float_called_with_double_int(4.4, 32);
}

struct Large {
  uint64_t a = 1;
  uint64_t b = 22;
  char c = 0x0c;
  uint64_t abcd[10]{0, 1, 2, 3, 4, 0, 0, 0, 0, 0};
  uint64_t d = 11;
  uint64_t e = 22;
};

struct Fits64Bits {
  uint32_t first{1};
  uint32_t second{2};
};

struct Fits128Bits {
  uint32_t first{1};
  uint32_t second{2};
  uint64_t third{3};
};


Fits64Bits passReturnByVal64Struct(Fits64Bits s) {
  s.first += 1;
  return s;
}

void pass128Struct(Fits128Bits s) {
  s.first += 1;
  s.second += s.third;
}


Large returnLarge(uint64_t x) {
  Large l;
  l.b = x;
  l.a = l.b / x;
  l.c = x * 1.68;
  return l;
}

char consumeLarge(Large l) { return l.c + l.a; }

size_t consumeString(std::string s) { return s.size(); }

size_t consumeStringRval(std::string &&s) { return s.size(); }

size_t consumeVec(std::vector<int> v) {
  assert(v.size() > 0);
  return v.size() + v[0];
}

template <typename T> T templateTest(T x) { return x; }

class CX {
private:
  struct NestedStruct {
    NestedStruct(CX &cx) : cx(cx) {};
    CX &cx;
    float pubNestBar(float f) {
      auto scrambler = [](float f) { return ((int)f) ^ 123456789; };
      return cx.pubFoo(cx.privBar(f)) + scrambler(f);
    }
  };
  NestedStruct *st = nullptr;

public:
  static int staticFn() { return 41; }

  std::string allTheStrings(std::string s1, std::string *s2,
                            const std::string &s3, std::string &&s4) {
    return s1;
  }

  std::string allTheStringsValNotFirst(std::string *s1, std::string &s2,
                                       std::string s3, std::string &&s4) {
    return s2;
  }

  std::string skipTwoArgsTest(std::string &str) { return str + "ooo"; }

  void publicString(std::string &str) {
    str.append("x");
    return;
  }

  void publicStringPtr(std::string *str) {
    str->append("x");
    return;
  }

  float nestedWrap() {
    if (st == nullptr) {
      st = new NestedStruct(*this);
    }

    return st->pubNestBar(49.1);
  }
  int pubFoo(float f) {

    std::cout << "f " << f << std::endl;

    {
      float f = 31;
      std::cout << "f2 " << f << std::endl;

      {
        float f = f * 2;
        std::cout << "f3 " << f << std::endl;
      }
    }

    return 0;
  }

private:
  int privBar(unsigned int x) { return x; }
};

long overload1(long x) { return x; }

unsigned long lotOfArgs(unsigned long a, unsigned long b, unsigned long c,
                        unsigned long d, unsigned long e, unsigned long f,
                        unsigned long g, long h, unsigned long i) {
  return a + b + c + d * e + f - g - h / i;
}

long overload1(short x) { return x; }

using MyTypeX = float;

using MyTypeT = MyTypeX;

MyTypeT myTypeTFoo(MyTypeT &ref) { return ref; }

auto abcd = [](auto x) { return x * 2; };

auto efgh = [](int x) { return x * 2; };

template <class T> void justPrint(T t) { std::cout << t << std::endl; }

namespace lambda_namespace {
auto namespaced_lambda = [](auto x) { return x * 2; };

float namespacedFnWithLambda(float f) {
  auto lmb = [](float x) { return x * 3.18 * x; };
  return lmb(f);
}
} // namespace lambda_namespace

float bignum(__uint128_t f) {
  return 0.0;
}

int main(int argc) {
  if (argc > 3) {
    *((volatile int*)0);
  }
  auto valStr = strByVal("hello, world!");
  if (argc > 2) {
    *((volatile int*)0);
  }
  lotOfArgs(1ul << 63, getInt(bignum(0xf0001)), valStr.length(), 4, 5, 6, 7, 8, 9);
  abcd(2);
  efgh(2);
  bignum(123);
  short num = 17;
  const char *p = "www";
  std::string v(p);

  for(int t = 0; t < 100; ++t){  
  foo_namespace::bar_namespace::foo(1, 3.14);
  std::cout << p << std::endl;
  templateTest<std::string>(v);
  templateTest<float>(0.0);
  }

  myTypeTFoo(retRef());
  MyTypeT x = 4.53;
  myTypeTFoo(x);
  if (argc > 1) {
    *((volatile int*)0);
  }
  overload1(overload1(num));

  auto nocapture_lam = [](int z) { return z; };

  auto valcapture_lam = [=](int &y) {
    y = 3;
    return x * 3;
  };

  CX c;
  auto refcapture_lam = [&](float *f) { return c.pubFoo(*f); };

  auto capture_cust_lam = [x, &num]() {
    num *= 2;
    return x + num;
  };

  nocapture_lam(0);
  int t = 1;
  valcapture_lam(t);

  float f = static_cast<float>(capture_cust_lam());
  refcapture_lam(&f);

  auto auto_lambda = [](auto x) { return x * 2; };

  float autofloat = auto_lambda(3.14f);
  int autoint = auto_lambda(static_cast<int>(12));
  addAuto(1, 2);
  justPrint(consumeLarge(Large{}));
  c.pubFoo(3.14);
  c.publicString(v);
  c.publicStringPtr(&v);
  std::string v2 = v + "1";
  std::string moving = v2 + "m";
  c.allTheStringsValNotFirst(&v, v2, v2 + "2", std::move(moving));
  moving = "moving2";
  c.allTheStrings(v2, &moving, "tmp1", "tmp2");
  auto sz = c.skipTwoArgsTest(v).size();
  CX::staticFn();
  sz = 1 + consumeStringRval("test");
  printf("Test value representation:");
  justPrint<char>(0xff);
  justPrint<unsigned char>(0xff);
  justPrint<short>(std::numeric_limits<short>::min());
  justPrint<unsigned short>(0xff0f);
  justPrint<int>(std::numeric_limits<int>::min());
  justPrint<unsigned int>(0xff0000ff);
  justPrint(std::numeric_limits<long long>::min());
  justPrint<unsigned long long>(0xffffffffffffffff);
  passReturnByVal64Struct(Fits64Bits());
  pass128Struct(Fits128Bits());
  return everything(sz) + lambda_namespace::namespaced_lambda(1) + autofloat +
         autoint + lambda_namespace::namespacedFnWithLambda(11.1) +
         c.nestedWrap();
}