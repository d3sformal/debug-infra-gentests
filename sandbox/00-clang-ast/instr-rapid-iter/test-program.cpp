#include <cassert>
#include <iostream>


namespace foo_namespace {
    namespace bar_namespace {
        void foo(int, float) {
            // just ReturnStmt (no children!)
            return;
        }
    }

    void baz(int i) {
        assert(i > 0);
    }
    
}

float x = 1;
float& retRef() {
    return x;
}

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

float float_called_with_double_int(double d, int i) {
    return d * i;
}

int everything(int ) {
    return int_called_with_int_float(0, 3.2f) + float_called_with_double_int(4.4, 32);
}

int main() {
    return everything(0);
}