#include <cassert>
#include <cstdio>
#include <iostream>

namespace foo_namespace {
namespace bar_namespace {
void foo(int, float) {
  // just ReturnStmt (no children!)
  return;
}
} // namespace bar_namespace

void baz(int i) { assert(i > 0); }

} // namespace foo_namespace

template<class T>
auto addAuto(T a, T b) {
  return a + b;
}

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

float float_called_with_double_int(double d, int i) { return d * i; }

int everything(int) {
  return int_called_with_int_float(0, 3.2f) +
         float_called_with_double_int(4.4, 32);
}

template <typename T> T templateTest(T x) { return x; }

class CX {
private:
  struct NestedStruct {
    NestedStruct(CX& cx) : cx(cx) {};
    CX& cx;
    float pubNestBar(float f) {
      auto scrambler = [](float f){
        return ((int)f) ^ 123456789;
      };
      return cx.pubFoo(cx.privBar(f)) + scrambler(f);
    }
  };
  NestedStruct* st = nullptr;
public:
  float nestedWrap() {
    if (st == nullptr) {
      st = new NestedStruct(*this);
    }
    
    return  st->pubNestBar(49.1);
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
  int privBar(int x) { return x; }
};

long overload1(long x) {
  return x;
}

long overload1(short x) {
  return x;
}

using MyTypeX = float;

using MyTypeT = MyTypeX;

MyTypeT myTypeTFoo(MyTypeT& ref) {
  return ref;
}

auto abcd = [](auto x) {
  return x * 2;
};

auto efgh = [](int x) {
  return x * 2;
};

namespace lambda_namespace {
  auto namespaced_lambda = [](auto x) {
    return x * 2;
  };

  float namespacedFnWithLambda(float f) {
    auto lmb = [](float x) {
      return x * 3.18 * x;
    };
    return lmb(f);
  }
}


int main() {

  abcd (2);
  efgh(2);

  short num = 17;
  foo_namespace::bar_namespace::foo(1, 3.14);
  templateTest<std::string>("");
  templateTest<float>(0.0);
  
  myTypeTFoo(retRef());
  MyTypeT x = 4.53;
  myTypeTFoo(x);

  overload1(overload1(num));

  auto nocapture_lam = [](int z){
    return z;
  };

  auto valcapture_lam = [=](int& y) {
    y = 3;
    return x * 3;
  };

  CX c;
  auto refcapture_lam = [&](float* f) {
    return c.pubFoo(*f);
  };

  auto capture_cust_lam = [x, &num]() {
    num *= 2;
    return x + num;
  };
  
  nocapture_lam(0);
  int t = 1;
  valcapture_lam(t);

  float f = static_cast<float>(capture_cust_lam());
  refcapture_lam(&f);


  auto auto_lambda = [](auto x) {
    return x * 2;
  };

  float autofloat = auto_lambda(3.14f);
  int autoint = auto_lambda(static_cast<int>(12));
  addAuto(1, 2);
  c.pubFoo(3.14);
  printf("Hellp!");
  return everything(0) + lambda_namespace::namespaced_lambda(1) + autofloat + autoint + lambda_namespace::namespacedFnWithLambda(11.1) + c.nestedWrap();
}