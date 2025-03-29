To compile and run everything (apart from LLVM):

`test-program.cpp`

    ./rebuild-pass.sh && ./run-pass.sh clang++ test-program.cpp && ./ir-to-bin.sh clang++ && ./export_lib_path.sh && ./build/a.out

`test-program.c`

    ./rebuild-pass.sh && ./run-pass.sh clang test-program.c && ./ir-to-bin.sh clang && ./export_lib_path.sh && ./build/a.out

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