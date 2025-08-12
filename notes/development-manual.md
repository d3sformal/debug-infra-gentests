# Development manual

This document aims to provide all the context necessary for extending and maintaining this project.
We suppose the reader is already familiar with all the core [concepts](../README.md#concepts) and
has at least attempted to run the [Docker/Podman container with the demo](../README.md#demo).

We will dive into 3 main areas:

1. Argument capture and argument type detection mechanisms
  * we provide a step-by-step guide for custom type support in the tool system
2. Development tips & tricks, troubleshooting techniques
3. Low-level details on the various communication mechanisms used in the tools developed in this project


## Argument capture and type detection mechanisms

Function arguments are being recorded and replaced when running a test. This requires at the very
least to be able to:

1. detect an argument's type (`T`)
2. inject code that serializes `T`
3. inject code that de-serializes `T`

In the following sections, we will explain how one may add support for a hypothetical type `T`.

### (De)serialization

Let us first focus on the (de)serialization of `T`. This is achieved by [`hooklib`](../sandbox/02-ipc/ipc-hooklib/). Concretely, the `GENFN_TEST_PRIMITIVE` macro and `llcap_hooklib_extra_cxx_string` functions in the [`hook.cpp`](../sandbox/02-ipc/ipc-hooklib/hook.cpp) file are the functions responsible for this.

The `GENFN_TEST_PRIMITIVE` creates a function definition with the appropriate body for a "primitive" data type. The function receives pointers to the source and target data, and two 4-byte values uniquely identifying the function (`module ID` and `function ID`), whose arguments are provided in the source pointer. A call to this function is inserted into every *target function* for every one of its arguments, effectively achieving something like this (injected lines marked with `//*`):

```c
void foo(int a, float b) {
  hook_arg_preamble(CONST_FOO_MOD_ID, CONST_FOO_FN_ID);   //*
  int a1;                                                 //*
  hook_int32(&a, &a1, CONST_FOO_MOD_ID, CONST_FOO_FN_ID); //*
  int b1;                                                 //*
  hook_float(&b, &b1, CONST_FOO_MOD_ID, CONST_FOO_FN_ID); //*

  // the original targetFoo where a is replaced by a1 and b is replaced by b1
}
```

Iniside `GENFN_TEST_PRIMITIVE`, if you ignore the `if (in_testing_mode)` block, you see the serializing part. In this part, we only require the `source` and `target` pointers. The `push_data` 
call abstracts sending data to the `llcap-server`. The pointer assignment right below it is an 
implementation detail of the approach to the instrumentation that we perform; we explain it in the 
following.

#### Why is the assignment to target needed?

As you can see in the example above, the `target` pointer will be later used as the "replaced" argument in the testing phase. As the argument capture and testing phases are merged (to reduce the number of recompilations),
the `target` remains to accommodate the testing phase. This is also why we skipped over the `if (in_testing_mode)` block.

#### Deserialization

In testing mode, we want to deserialize an argument the **test coordinator** provides to us.
Remember, only the *child* of the `fork`ed coordinator is the execution flow where arguments of an
n-th call should be replaced, and we have to ensure not to replace in other calls.

The exact mechanism of the `fork`ing is thus:

1. The test coordinator is spawned, it is the instrumented binary running seemingly without any change - assume its parameters are that we are testing the function `foo`'s 3rd call 
2. Test coordinator approaches target function `foo`
3. In each target function (not just `foo`), the `hook_arg_preamble` is called, for the first call of `foo`, only `register_call` is called, and `should_hijack_arg` indicates that we should wait a few more calls
4. For each argument of `foo`, the "deserialization" hook function is called; each of those, however, checks whether the `should_hijack_arg` holds, and if not, we ensure arguments are not replaced
5. On another entry of `foo`, `should_hijack_arg` once again indicates we should wait with argument replacement; the argument hooks once again do nothing
6. The third (desired) call of `foo` flips `should_hijack_arg` to true and `perform_testing` is called

`perform_testing` is a no-return function that always exits, effectively terminating the test coordinator.

Looking again at the `GENFN_TEST_PRIMITIVE`, now inside the block where `in_testing_mode` holds. 
Since we are in the testing fork and `should_hijack_arg` holds (point 6 above), we 
register an argument (required step for internal bookkeeping of how many arguments are to be sent)
and consume some bytes from an incoming argument packet. This argument packet is "magically" there
and is supposed to be the precise size of all arguments that we receive. We use the `target` 
pointer to write the binary data into the new argument and return. We can expect that what is 
written in `*target` will be used as the argument of `foo`. Notice that if any of the checks inside 
the function fail, the `target` is filled with the original argument (`argvar`) anyway. It is 
crucial to ensure this step.

#### Deserialization of dynamic-sized types

For custom types of dynamic sizes, you must push first the 8-byte size of the payload, followed by
your custom payload. In deserialization, you will receive exactly the custom payload (without the 
first 8-byte size). Furthermore, the dynamic-sized types receive a pointer **to a pointer to `T`** 
and it is the **responsibility of the deserializer to initialize this pointer to `T`** (i.e., to allocate
a new object of type `T`).

For a dynamic-sized example, see the `llcap_hooklib_extra_cxx_string` function.

For fixed-sized types, up to 16 bytes (see [relevant hooklib header](../sandbox/01-llvm-ir/llvm-pass/src/typeids.h)).

In summary, for custom type serialization and deserialization, you must:

1. pick a specific-enough function name (to avoid collisions)
2. declare the function in the `hook.h` file (make sure C++ code uses `extern "C"`!)
3. define the function in the `hook.cpp` file
  * ensure you separate the serialization and deserialization by checking `in_testing_mode`
  * serialize based on the target type size (static up to 16B vs dynamic)
  * deserialize only in the call you're supposed to (by checking `is_fn_under_test` and `should_hijack_arg`)
    * call `register_argument`
    * use `consume_bytes_from_packet` to receive your data
  * **always** deserialize into the `target` pointer - `*target` must point to an object of `T` even if argument replacement has not taken place

What remains is to rebuild `hooklib` by navigating to its [directory](../sandbox/02-ipc/ipc-hooklib/) and running `make`.

### Injection of code, detection of data types

With our (de)serialization code written, we now need to provide information to compiler plugins to 
detect the type `T` and insert calls to our (de)serialization.

We need to know the `T`'s AST type name and its size (as used in the serialization function written earlier). You can use `clang -Xclang -ast-dump main.c` (or with the `-fsyntax-only` flag) to discover the AST type name of your types.

#### AST plugin extension

To add support for `T` on the AST plugin's side, you must modify the AST plugin and rebuild it (for now). During the build process, the [AST plugin's directory](../sandbox/01-llvm-ir/custom-metadata-pass/ast-meta-add/) is copied into the [`llvm-project` file structure](../sandbox/llvm-project/clang/examples/ast-meta-add/). 
Be sure to either copy the modified files to this folder, or perform modifications directly inside 
it. For clarity, we will refer to the non-llvm-project files in the file links that follow.

1. `#define` a new unique string literal in the [`llvm-metadata.h`](../sandbox/01-llvm-ir/custom-metadata-pass/ast-meta-add/llvm-metadata.h) file. This will serve as a metadata key used to identify the type `T`.
2. inside the `addFunctionLocationMetadata` function, add a call to `encodeArgIndiciesSatisfying` similar to the snippet below, right where `encodeArgIndiciesSatisfying` is first used

```c++
encodeArgIndiciesSatisfying(
    // pass metadata key
    <your #defined literal>, 
    FD, // clang's Function Declaration object
    // lambda inspecting an argument at an index
    // It returns true if the Arg "corresponds" to the type T (your metadata key)
    [](ParmVarDecl *Arg, size_t Idx) {
      auto TypeName = Arg->getType().getCanonicalType().getAsString();
      return /* a boolean expression */
    }
);
```

To help with the instrumentation of pointer-based (pointer, references) types, you can use
`isTargetTypeValRefPtr` function.

**Note**: only functions defined in the compiled sources (that are not external to the compilation process) are instrumented!

Now, head to the `llvm-project` build directory, run `ninja -j $(nproc)` and `ninja install`.

#### IR plugin extension

The LLVM IR is the level where our instrumentation is performed. We use the information relayed 
from the AST plugin to guide our instrumentation. In the previous section, we created a *metadata key* and instructed the AST plugin to mark functions accepting `T` (or pointers/references to `T`).

Now we will make the LLVM IR plugin aware of the new *metadata key* **as well as** the *(de)serialization* function we've written in `hooklib` [earlier](#deserialization).

First, ensure your modifications made to the `llvm-metadata.h` file are available in the non-llvm-project file ([`llvm-metadata.h`](../sandbox/01-llvm-ir/custom-metadata-pass/ast-meta-add/llvm-metadata.h)). Next, head to the [`instrumentation.cpp`](../sandbox/01-llvm-ir/llvm-pass/src/instrumentation.cpp) file and modify the `SCustomSizes` and `SCustomHooks` (namespace `common`)
in the following way:

In `SCustomSizes`, add a tuple (`{ }`) that maps your custom metadata key to an `LlcapSizeType` (defined in [here](../sandbox/01-llvm-ir/llvm-pass/src/typeids.h)). Use the `LLSZ_8` up to `LLSZ_128` for 8 to 128-bit **value** types and `LLSZ_CUSTOM` for a dynamic-sized type.
Remember that this choice has already been made in the [(de)serialization function](#deserialization) you've written earlier for type `T`.

In `SCustomHooks`, you once again map your custom metadata key, this time to the structure
`SCustomTypeDescription{ m_hookFnName, m_log_name }` where `m_hookFnName` is the name of the 
the [(de)serialization function](#deserialization) you've created earlier. and `m_log_name` is *whatever* textual description you'd like the instrumentation to have.

Now, compile the plugin by running `make` in the [`llvm-pass` directory](../sandbox/01-llvm-ir/llvm-pass/).

Now, when you re-compile your program using `T` and instrument the proper function, you should see `T` being captured by the `llcap-server`.
