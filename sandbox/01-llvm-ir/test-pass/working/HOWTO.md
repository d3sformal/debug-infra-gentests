To compile and run everything (apart from LLVM):

`test-program.cpp`

    ./build-and-run.sh clang++ test-program.cpp

`test-program.c`

    ./build-and-run.sh clang test-program.c

You can also append `-mllvm -llcap-verbose` option.

Example output for `test-program.cpp`:

```
[HOOK] start: main
[HOOK] start: std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> templateTest<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>)
[HOOK] start: float templateTest<float>(float)
[HOOK] float: 0.000000
[HOOK] start: CX::pubFoo(float)
[HOOK] float: 3.140000
f 3.14
f2 31
f3 6.18982e-41
Hellp![HOOK] start: everything(int)
[HOOK] int: 0
[HOOK] start: int_called_with_int_float(int, float)
[HOOK] int: 0
[HOOK] float: 3.200000
[HOOK] start: float_called_with_double_int(double, int)
[HOOK] double: 4.400000
[HOOK] int: 32
```

Example output for `test-program.c`:

```
[HOOK] start: main
[HOOK] start: foo
[HOOK] int: 3
C FOO: 3
[HOOK] start: baz
[HOOK] int: 1
[HOOK] float: 2.710000
C BAZ: 1 2.710000
[HOOK] start: foo
[HOOK] int: 1
C FOO: 1
[HOOK] start: bar
[HOOK] float: 2.710000
C BAR: 2.710000
[HOOK] start: doubleBaz
[HOOK] int: 1
[HOOK] double: 3.141590
C DOUBLE BAZ: 1 3.141590
[HOOK] start: foo
[HOOK] int: 1
C FOO: 1
[HOOK] start: bar
[HOOK] float: 3.141590
C BAR: 3.141590
[HOOK] start: constStructFunc
```

For integration with a custom metadata-generating plugin, you can run

        ./build-and-run-meta.sh clang++ test-program.cpp

Again, you may append `-mllvm -llcap-verbose`. To run metadata plugin but not use it to determine instrumentation, use `-mllvm -llcap-filter-by-mangled`.
<details>
<summary>
Example output (click to view):
</summary>



```
[HOOK] start: main
[HOOK] start: lotOfArgs(unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, long, unsigned long)
[HOOK] unsigned long long: 9223372036854775808
[HOOK] unsigned long long: 2
[HOOK] unsigned long long: 3
[HOOK] unsigned long long: 4
[HOOK] unsigned long long: 5
[HOOK] unsigned long long: 6
[HOOK] unsigned long long: 7
[HOOK] long long: 8
[HOOK] unsigned long long: 9
[HOOK] start: auto $_1::operator()<int>(int) const
[HOOK] int: 2
[HOOK] start: $_4::operator()(int) const
[HOOK] int: 2
[HOOK] start: foo_namespace::bar_namespace::foo(int, float)
[HOOK] int: 1
[HOOK] float: 3.140000
www
[HOOK] start: std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> templateTest<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>)
[HOOK] std::string: www
[HOOK] start: float templateTest<float>(float)
[HOOK] float: 0.000000
[HOOK] start: retRef()
[HOOK] start: myTypeTFoo(float&)
[HOOK] start: myTypeTFoo(float&)
[HOOK] start: overload1(short)
[HOOK] short: 17
[HOOK] start: overload1(long)
[HOOK] long long: 17
[HOOK] start: main::$_1::operator()(int) const
[HOOK] int: 0
[HOOK] start: main::$_2::operator()(int&) const
[HOOK] start: main::$_3::operator()() const
[HOOK] start: main::$_4::operator()(float*) const
[HOOK] start: CX::pubFoo(float)
[HOOK] float: 38.529999
f 38.53
f2 31
f3 9.17038e-41
[HOOK] start: _ZZ4mainENK3$_0clIfEEDaT_
[HOOK] float: 3.140000
[HOOK] start: _ZZ4mainENK3$_0clIiEEDaT_
[HOOK] int: 12
[HOOK] start: auto addAuto<int>(int, int)
[HOOK] int: 1
[HOOK] int: 2
[HOOK] start: consumeLarge(Large)
[HOOK] start: void justPrint<char>(char)
[HOOK] byte: 13

[HOOK] start: CX::pubFoo(float)
[HOOK] float: 3.140000
f 3.14
f2 31
f3 6.14189e-41
[HOOK] start: CX::publicString(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&)
[HOOK] std::string: www
[HOOK] start: CX::publicStringPtr(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*)
[HOOK] std::string: wwwx
[HOOK] start: CX::allTheStringsValNotFirst(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&)
[HOOK] std::string: wwwxx
[HOOK] std::string: wwwxx1
[HOOK] std::string: wwwxx12
[HOOK] std::string: wwwxx1m
[HOOK] start: CX::allTheStrings(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> const&, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&)
[HOOK] std::string: wwwxx1
[HOOK] std::string: moving2
[HOOK] std::string: tmp2
[HOOK] start: CX::skipTwoArgsTest(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&)
[HOOK] std::string: wwwxx
[HOOK] start: CX::staticFn()
[HOOK] start: consumeStringRval(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>&&)
[HOOK] std::string: test
Test value representation:[HOOK] start: void justPrint<char>(char)
[HOOK] byte: -1
�
[HOOK] start: void justPrint<unsigned char>(unsigned char)
[HOOK] unsigned byte: 255
�
[HOOK] start: void justPrint<short>(short)
[HOOK] short: -32768
-32768
[HOOK] start: void justPrint<unsigned short>(unsigned short)
[HOOK] unsigned short: 65295
65295
[HOOK] start: void justPrint<int>(int)
[HOOK] int: -2147483648
-2147483648
[HOOK] start: void justPrint<unsigned int>(unsigned int)
[HOOK] unsigned int: 4278190335
4278190335
[HOOK] start: void justPrint<long long>(long long)
[HOOK] long long: -9223372036854775808
-9223372036854775808
[HOOK] start: void justPrint<unsigned long long>(unsigned long long)
[HOOK] unsigned long long: 18446744073709551615
18446744073709551615
[HOOK] start: passReturnByVal64Struct(Fits64Bits)
[HOOK] long long: 0
[HOOK] start: everything(int)
[HOOK] int: 5
[HOOK] start: int_called_with_int_float(int, float)
[HOOK] int: 0
[HOOK] float: 3.200000
[HOOK] start: float_called_with_double_int(double, int)
[HOOK] double: 4.400000
[HOOK] int: 32
[HOOK] start: auto lambda_namespace::$_3::operator()<int>(int) const
[HOOK] int: 1
[HOOK] start: lambda_namespace::namespacedFnWithLambda(float)
[HOOK] float: 11.100000
[HOOK] start: lambda_namespace::namespacedFnWithLambda(float)::$_0::operator()(float) const
[HOOK] float: 11.100000
[HOOK] start: CX::nestedWrap()
[HOOK] start: CX::NestedStruct::NestedStruct(CX&)
[HOOK] start: CX::NestedStruct::pubNestBar(float)
[HOOK] float: 49.099998
[HOOK] start: CX::privBar(unsigned int)
[HOOK] unsigned int: 49
[HOOK] start: CX::pubFoo(float)
[HOOK] float: 49.000000
f 49
f2 31
f3 9.18271e-41
[HOOK] start: CX::NestedStruct::pubNestBar(float)::'lambda'(float)::operator()(float) const
[HOOK] float: 49.099998
```

**C version**

```
[HOOK] start: main
[HOOK] start: foo
[HOOK] int: 3
C FOO: 3
[HOOK] start: typedefConsumer1
[HOOK] start: constStructFunc
[HOOK] start: typedefConsumer2
[HOOK] start: constStructFunc
[HOOK] start: baz
[HOOK] int: 1
[HOOK] float: 2.710000
C BAZ: 1 2.710000
[HOOK] start: foo
[HOOK] int: 1
C FOO: 1
[HOOK] start: bar
[HOOK] float: 2.710000
C BAR: 2.710000
[HOOK] start: doubleBaz
[HOOK] int: 1
[HOOK] double: 3.141590
C DOUBLE BAZ: 1 3.141590
[HOOK] start: foo
[HOOK] int: 1
C FOO: 1
[HOOK] start: bar
[HOOK] float: 3.141590
C BAR: 3.141590
[HOOK] start: constStructFunc
End
```

</details>