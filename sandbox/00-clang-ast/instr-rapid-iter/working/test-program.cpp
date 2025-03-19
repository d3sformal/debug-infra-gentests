#include "../../inject-w-library/lib/include/funTrace.hpp"
#include <cassert>
#include <iostream>

void foo(int, float) {
  auto __function_tracking_scope = funTraceLib::ScopeDumper("foo", 1);

  // just ReturnStmt (no children!)
  __function_tracking_scope.registerReturn();
  return;
}

float x = 1;
float &retRef() { return x; }

void baz(int i) {
  auto __function_tracking_scope = funTraceLib::ScopeDumper("baz", 2);

  assert(i > 0);
  __function_tracking_scope.registerReturn();
}

int int_called_with_int_float(int i, float f) {
  auto __function_tracking_scope =
      funTraceLib::ScopeDumper("int_called_with_int_float", 3);

  // Var (x, builtintype::int) -> ImplicitCast
  int x = i * f;

  // Var (y, auto) -> -> BinaryOperator
  auto y = i * f;
  // ReturnStmt -> ImplicitCast -> BinaryOperator
  auto &&__function_tracking_retval_0(i * f);

  __function_tracking_scope.registerReturn();
  return __function_tracking_retval_0;
  ;
  // ImplicitCast -> BinaryOperator
  auto &&__function_tracking_retval_1(y);

  __function_tracking_scope.registerReturn();
  return __function_tracking_retval_1;
  ;

  // =>
  // return (stmt);

  // should be transformable to

  // auto x = (stmt);
  // ... instrument fn return ...
  // return x;

  // If -> Compound -> Return
  if (true) {
    auto &&__function_tracking_retval_2(x);

    __function_tracking_scope.registerReturn();
    return __function_tracking_retval_2;
    ;
  }

  // If -> Return
  if (false) {
    auto &&__function_tracking_retval_3(y);

    __function_tracking_scope.registerReturn();
    return __function_tracking_retval_3;
  };
  if (false) {
    auto &&__function_tracking_retval_4(y);

    __function_tracking_scope.registerReturn();
    return __function_tracking_retval_4;
  };

  // NOT part of compound expression?
  // replace Return with Compound, insert Return into Compound
  // replace

  auto &&__function_tracking_retval_5(retRef());

  __function_tracking_scope.registerReturn();
  return __function_tracking_retval_5;
  ;
}

float float_called_with_double_int(double d, int i) {
  auto __function_tracking_scope =
      funTraceLib::ScopeDumper("float_called_with_double_int", 4);

  auto &&__function_tracking_retval_0(d * i);

  __function_tracking_scope.registerReturn();
  return __function_tracking_retval_0;
  ;
}

int everything(int) {
  auto __function_tracking_scope = funTraceLib::ScopeDumper("everything", 5);

  auto &&__function_tracking_retval_0(int_called_with_int_float(0, 3.2f) +
                                      float_called_with_double_int(4.4, 32));

  __function_tracking_scope.registerReturn();
  return __function_tracking_retval_0;
  ;
}

int main() {
  auto __funtraceLibLogger = funTraceLib::TraceLogger("./log.txt");

  return everything(0);
}