# IPC experiments

This folder contains various components aimed at implementing a working and viable solution for the problem of IPC between an instrumented application and a "test driver" application.

Aims:

1. receive & process function call information from the instrumented program
2. export of function identifiers selected by the user - these are the targets which we want to test
3. capture of function argument data
4. ability to "send" captured information to the forked tested processes

## Components

* [`ipc-finalizer`](./ipc-finalizer/) - a tiny application aimed at indicating program crashes to the test driver
* [`ipc-hooklib`](./ipc-hooklib/) - hook library that is able to communicate via selected IPC method with the "test driver" (similar to [the first hook library version](../01-llvm-ir/test-pass/hooklib/))
* [`llcap-server`](./llcap-server/) - The "test driver" Rust application, recipient of (function call) information 
* [`example-complex`](./example-complex/) - cmake-managed test "project" to be instrumented (for testing a build system integration as well as multiple-file projects)

## Demo

To capture function calls from the `example-complex` application, perform:

    cd ./example-complex
    ./build.sh
    cd ../llcap-server
    cargo r -- --modmap=../example-complex/module-maps/ shmem -f "/llcap-"

Now in another terminal (inside this folder)

    cd ./example-complex
    ./example

On the `llcap-server`-side you should see a summary of calls printed, something like this:

<details>
<summary>Expand (example inflated with a 1000000 extra loops of 3 calls)</summary>

Format:

`Function ID - number of calls - demangled function name (module name)`

```
0 - 1000000 - std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> templateTest<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
1 - 1000000 - foo_namespace::bar_namespace::foo(int, float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
2 - 1000000 - float templateTest<float>(float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
3 - 3 - CX::pubFoo(float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
4 - 2 - myTypeTFoo(float&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
5 - 2 - void justPrint<char>(char) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
6 - 2 - bignum(unsigned __int128) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
7 - 1 - void justPrint<int>(int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
8 - 1 - main::$_4::operator()() const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
9 - 1 - void justPrint<unsigned int>(unsigned int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
10 - 1 - pass128Struct(Fits128Bits) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
11 - 1 - void justPrint<short>(short) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
12 - 1 - int_called_with_int_float(int, float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
13 - 1 - CX::publicString(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
14 - 1 - main::$_1::operator()(int) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
15 - 1 - auto addAuto<int>(int, int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
16 - 1 - CX::allTheStrings(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> const&, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
17 - 1 - main::$_3::operator()(float*) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
18 - 1 - void justPrint<long long>(long long) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
19 - 1 - everything(int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
20 - 1 - auto $_1::operator()<int>(int) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
21 - 1 - overload1(short) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
22 - 1 - overload1(long) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
23 - 1 - _ZZ4mainENK3$_0clIfEEDaT_ (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
24 - 1 - main (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
25 - 1 - CX::staticFn() (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
26 - 1 - retRef() (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
27 - 1 - CX::skipTwoArgsTest(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
28 - 1 - consumeStringRval(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
29 - 1 - auto lambda_namespace::$_3::operator()<int>(int) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
30 - 1 - void justPrint<unsigned short>(unsigned short) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
31 - 1 - passReturnByVal64Struct(Fits64Bits) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
32 - 1 - getInt(float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/lib.cpp)
33 - 1 - getCoeff(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> const&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/lib.cpp)
34 - 1 - float_called_with_double_int(double, int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
35 - 1 - $_4::operator()(int) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
36 - 1 - lambda_namespace::namespacedFnWithLambda(float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
37 - 1 - CX::NestedStruct::pubNestBar(float)::'lambda'(float)::operator()(float) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
38 - 1 - void justPrint<unsigned long long>(unsigned long long) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
39 - 1 - lambda_namespace::namespacedFnWithLambda(float)::$_0::operator()(float) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
40 - 1 - CX::nestedWrap() (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
41 - 1 - consumeLarge(Large) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
42 - 1 - void justPrint<unsigned char>(unsigned char) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
43 - 1 - CX::NestedStruct::pubNestBar(float) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
44 - 1 - lotOfArgs(unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, long, unsigned long) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
45 - 1 - _ZZ4mainENK3$_0clIiEEDaT_ (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
46 - 1 - strByVal(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/lib2.cpp)
47 - 1 - CX::NestedStruct::NestedStruct(CX&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
48 - 1 - CX::allTheStringsValNotFirst(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
49 - 1 - CX::privBar(unsigned int) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
50 - 1 - main::$_2::operator()(int&) const (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
51 - 1 - CX::publicStringPtr(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*) (module /home/bohdanqq/source/research-project/sandbox/02-ipc/example-complex/main.cpp)
Module map summary:
Total Modules loaded: 7
Total Functions loaded: 76
Total traced calls: 3000054
Traces originated from 3 modules
```
</details>