# Notes on LLVM IR metadata

## Aims

Map the possibilities of LLVM IR metadata, specifically:

1. explore IR **metadata** capabilities wrt. **any function** (as a single entity)
2. explore IR **metadata** capabilities wrt. **a function's parameters** (e.g. tying metadata to specific function's arguments, all arguments, accross C++'s template instantiations, etc.)
3. discuss/decide **feasability** of exporting custom metadata from pre-IR-generating LLVM/Clang passes into the IR
4. the minimum outcome is deeper understanding of the LLVM architecture, LLVM IR syntax and semantics

## Concrete Why-s

**WHY1**: *LLVM pass, function "classification":* in clang's AST Matchers, we can query whether a function is built-in / library /`#include`d, in LLVM IR, this information is not available (not in a stable way)

**WHY2**: *LLVM pass, function argument capture:* exporting specific type information into the IR would allow the LLVM IR pass and accompanying library to support more types, easing extensions of the envisioned tools

* ??? who knows what we'll find!


## Entrypoint/Low-hanging fruit

* [StackOverflow post - TO BE VERIFIED](https://stackoverflow.com/questions/19743861/what-is-llvm-metadata)
    * > The main "client" of metadata in LLVM is currently debug info. It's used by the front-end (e.g. Clang) to tag the LLVM IR it generates with debug information that correlates IR to the source code it came from. 
    * [link to LLVM docs](https://llvm.org/docs/LangRef.html#metadata)
    * [link to an introductory post by the LLVM author himself](https://blog.llvm.org/2010/04/extensible-metadata-in-llvm-ir.html)

### Introductory post

* motivations seem to overlap: **extensible**, for **front-end** writers to do whatever they want, should not interfere with optimizations and should be easily **decodable**
* Links
    * http://llvm.org/docs/LangRef.html#metadata
    * http://llvm.org/docs/LangRef.html#namedmetadatastructure
* *metadata is not a first-class type*
    * operand of an instrinsic or an operand to another metadata or attached to an instruction 
* `MDString` - string data
    * The `MDString` class allows C++ code walking the IR to **access** the **arbitrary string data** with a `StringRef` - Bingo?
    * `!"foo"`
* `MDNode` - tuple that can rceference arbitrary LLVM IR values in the program as well as other metadata
    * `!23 = !{ i32 4, !"foo", i32 *@G, metadata !22 }`
    * `ConstantInt`, `MDString`, global variable, `MDNode`
* two types, global and **function-local**
    *  *a function-local MDNode can (potentially transitively) refer to instructions within a particular function*
* `NamedMDNode` - named access at module level , "finding metadata by name"
    * `Module`class - list of `NamedMDNode`
    *  `!my_named_mdnode = !{ !1, !2, !4212 }`
    * *In this case, the code generator uses this information to know that the metadata !0 is the variable descriptor for the alloca %X. Note that **intrinsics** themselves are not considered metadata, so they **can affect code generation etc***

#### LLVM instrinsics referencing metadata

* an obvious TODO

```
!0 = metadata !{i32 524544, ...

...
  %x = alloca i32
  call void @llvm.dbg.declare(metadata !{i32* %x}, metadata !0)
```
* `!0` passes the module-level !0 MDNode into the second argument and passes a function-local MDNode as the first argument (which, since it is an mdnode, does not count as a use of %X)
    * no metadata use is counted as use of that IR object

#### Metadata attached to instructions

```
store i32 0, i32* %P, !nontemporal !2, !frobnatz !123

ret void, !dbg !9
```

#### Important point

> A potential future use case is to support Type-Based Alias Analysis (TBAA). TBAA is an optimization to know that "float *P1" and "int *P2" can never alias (in GCC, this is enabled with -fstrict-aliasing). The trick with this is that it isn't safe to implement TBAA in terms of LLVM IR types, you really need to be able to encode and express a type-subset graph according to the complex source-level rules (e.g. in C, "char*" can alias anything).

Custom analysis of such "aliasing" pointers could yield invalid programs (if `char *` aliases non-null-terminated data, for example, sending it to our library would cause huge issues)

### Documentation

* `DISubprogram` nodes that represent functions contain unmangled functuion names

## Exploration

When compiled **with debug information** (`-g`), `DISubprogram` is available via:

```c++
if (DISubprogram* subprogram = F.getSubprogram(); subprogram) {
    subprogram->dump();
} 
```

A possible hint into the function's origin (as per **WHY1**) is the `subprogram->getFilename()`.

**Examples**

* the file name output is the last line of each snippet

Example template function:

```
_Z12templateTestIfET_S0_
<0x426b8de0> = distinct !DISubprogram(name: "templateTest<float>", linkageName: "_Z12templateTestIfET_S0_", scope: <0x41526940>, file: <0x41526940>, line: 65, type: <0x426ce8c0>, scopeLine: 65, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x4154e558>, templateParams: <0x426cbdc8>, retainedNodes: <0x421b8de0>)
test-program.cpp
```

Example member function

```
<0x4153bfa0> = distinct !DISubprogram(name: "pubFoo", linkageName: "_ZN2CX6pubFooEf", scope: <0x41a2b328>, file: <0x41526940>, line: 69, type: <0x4253ae70>, scopeLine: 69, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x4154e558>, declaration: <0x4253af00>, retainedNodes: <0x421b8de0>)
test-program.cpp
```

Example template funcion and a destructor (both coming out of the library)

```
_ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEC2IS3_EEPKcRKS3_
<0x423243b0> = distinct !DISubprogram(name: "basic_string<std::allocator<char> >", linkageName: "_ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEC2IS3_EEPKcRKS3_", scope: <0x42642508>, file: <0x418f1340>, line: 646, type: <0x426422c0>, scopeLine: 648, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x4154e558>, templateParams: <0x418f14b8>, declaration: <0x42649ec0>, retainedNodes: <0x421b8de0>)
/usr/lib/gcc/x86_64-redhat-linux/14/../../../../include/c++/14/bits/basic_string.h

_ZNSaIcED2Ev
<0x4263c490> = distinct !DISubprogram(name: "~allocator", linkageName: "_ZNSaIcED2Ev", scope: <0x4229a8c8>, file: <0x42299db0>, line: 182, type: <0x4229a700>, scopeLine: 182, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x4154e558>, declaration: <0x420ef360>, retainedNodes: <0x421b8de0>)
/usr/lib/gcc/x86_64-redhat-linux/14/../../../../include/c++/14/bits/allocator.h
```

One option is to compare the `subprogram->getFilename()` path to the files of all compiled files (by parsing e.g. `compile_commands.json` that can be generated). Or we could "export" the paths used to look up library functions (note that this is different than "all include paths"). It could be tricky, though to generalize the approach to work accross toolchains (Make vs CMake) and filesytem structures (links, more complex directory structure, ...).

### Metadata of overloaded functions

Considering:

```c++
long overload1(long x) {
  return x;
}

long overload1(short x) {
  return x;
}
```

The metadata generated is as follows:

```
// overload1(long)
<0x24ca40e0> = distinct !DISubprogram(name: "overload1", linkageName: "_Z9overload1l", scope: <0x23c9f940>, file: <0x23c9f940>, line: 90, type: <0x240590c0>, scopeLine: 90, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x23cc75d8>, retainedNodes: <0x243df690>)

// overload1(short)
<0x24c9df10> = distinct !DISubprogram(name: "overload1", linkageName: "_Z9overload1s", scope: <0x23c9f940>, file: <0x23c9f940>, line: 94, type: <0x24c13580>, scopeLine: 94, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: <0x23cc75d8>, retainedNodes: <0x243df690>)
```

Notice that `name` is just a name. There is no demangling going on. Dumping the only differing
metadata: `type`:

```
overload1(long)
<0x37f288c0> = !DISubroutineType(types: <0x37963960>)
  <0x37963960> = !{<0x3778c7b8>, <0x3778c7b8>} // 2 references to the same object (long type metadata)
    <0x3778c7b8> = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)

overload1(short)
<0x385ca030> = !DISubroutineType(types: <0x3865a0f0>)
  <0x3865a0f0> = !{<0x3778c7b8>, <0x386531a8>}
    <0x3778c7b8> = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
    <0x386531a8> = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
```

#### notable discrepancy when using `clang` vs `clang++` (to keep in mind)

output of execution (accidentally) compiled with `clang++`:

```
[HOOK] start: main
[HOOK] start: foo(int)
[HOOK] int: 3
C FOO: 3
[HOOK] start: baz(int, float)
...
```

now with `clang`:

```
[HOOK] start: main
[HOOK] start: foo
[HOOK] int: 3
C FOO: 3
[HOOK] start: baz
...
```
The function names printed out come from demangling the function names. Obviously the `C` compiler does not mangle the names, therefore there is less information in the final function name present in the IR. 

This further hints at the need to inspect metadata to ensure that uniquely identifying information about functions is exported/retained after instrumentation.

This means multiple things:

We should query this metadata for **precise** argument type information (in addition to the IR type)!

There is a possibility of creating custom function descriptions as we can avoid parsing the demangled name (`foo(int, short, char*)`). What remains to be seen is the behaviour of templated code, `class`es and `typedef`s.

#### Templates, Classes, Typedefs

* **Template functions** - expose the same argument metadata as normal functions (example for `templateTest<float>`)

```
<0x3bf41cf0> = !DISubroutineType(types: <0x3bf41ca0>)
  <0x3bf41ca0> = !{<0x3adbac48>, <0x3adbac48>}
    <0x3adbac48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
```

* **using** type aliases

Function `MyTypeT myTypeTFoo(MyTypeT&)` uses a alias `using MyTypeT = float`, the metadata looks thus:

```
<0x20f53140> = !DISubroutineType(types: <0x20f4e590>)
  <0x20f4e590> = !{<0x20f53050>, <0x20f530d0>}
    <0x20f53050> = !DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeT", file: <0x1ff51940>, line: 65, baseType: <0x1ff75c48>)
      <0x1ff51940> = !DIFile(filename: "test-program.cpp", directory: "SNIPPED", checksumkind: CSK_MD5, checksum: "b6ae8206e3dcc197329da2927c91a308")
      <0x1ff75c48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
    <0x20f530d0> = !DIDerivedType(tag: DW_TAG_reference_type, baseType: <0x20f53050>, size: 64)
```

* nested **using** type alias

setup:

```c++
using MyTypeX = float;

using MyTypeT = MyTypeX;

MyTypeT myTypeTFoo(MyTypeT& ref) {
  return ref;
}
```

Metadata:

```
<0x130a4cb0> = !DISubroutineType(types: <0x1309ffe0>)
  <0x1309ffe0> = !{<0x130a4bc0>, <0x130a4c40>}
    <0x130a4bc0> = !DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeT", file: <0x120a4940>, line: 100, baseType: <0x130a4b40>)
      <0x120a4940> = !DIFile(SNIPPED, SEE ABOVE)
      <0x130a4b40> = !DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeX", file: <0x120a4940>, line: 98, baseType: <0x120c8c48>)
        <0x120c8c48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
    <0x130a4c40> = !DIDerivedType(tag: DW_TAG_reference_type, baseType: <0x130a4bc0>, size: 64)
```

* `C` `typedef`s - from my POV they behave the same as `using`s

* **classes - member functions**

Inspecting `pubFoo`:

```
<0x2e2016b0> = !DISubroutineType(types: <0x2e201668>)
  <0x2e201668> = !{<0x2d2f2ba8>, <0x2e201600>, <0x2d20ec48>}
    <0x2d2f2ba8> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
    <0x2e201600> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x2d38fa18>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
      <0x2d38fa18> = distinct !DICompositeType(tag: DW_TAG_class_type, name: "CX", SNIP CX METADATA)
    <0x2d20ec48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
```

We clearly see the `this` pointer passed as `DW_TAG_pointer_type` with flags `DIFlagArtificial | DIFlagObjectPointer` and the correspondign class type (`CX`).

#### Lambdas

As I have forgotten about instrumenting lambdas in general, this section will have more information regarding them. See [test-program.cpp](../sandbox/01-llvm-ir/test-pass/test-program.cpp)

The dump library skeleton reports their corresponding `operator()`:

```
[HOOK] start: main::$_0::operator()(int) const
[HOOK] int: 0
[HOOK] start: main::$_1::operator()(int&) const
[HOOK] start: main::$_2::operator()() const
[HOOK] start: main::$_3::operator()(float*) const
```

The following is the metadata (`types`):

```
// Noncapturing lambda that takes and returns an `int`

_ZZ4mainENK3$_0clEi main::$_0::operator()(int) const
<0x33511980> = distinct !DISubprogram(name: "operator()", linkageName: "_ZZ4mainENK3$_0clEi", scope: <0x33195e78>, file: <0x324ec940>, line: 119, type: <0x335105a0>, scopeLine: 119, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x32514848>, declaration: <0x33511900>, retainedNodes: <0x32692020>)
test-program.cpp
<0x325f4d48> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
<0x32f80790> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x33577750>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
<0x325f4d48> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)

// capture-by-value lambda that takes an `int&`, return `MyTypeT`

_ZZ4mainENK3$_1clERi main::$_1::operator()(int&) const
<0x32c56a50> = distinct !DISubprogram(name: "operator()", linkageName: "_ZZ4mainENK3$_1clERi", scope: <0x3278b018>, file: <0x324ec940>, line: 123, type: <0x33511ca0>, scopeLine: 123, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x32514848>, declaration: <0x32c569d0>, retainedNodes: <0x32692020>)
test-program.cpp
<0x334e98c0> = !DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeT", file: <0x324ec940>, line: 100, baseType: <0x334e9840>)
<0x3350f9b0> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x3350f8b0>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
<0x32f80c90> = !DIDerivedType(tag: DW_TAG_reference_type, baseType: <0x325f4d48>, size: 64)


// lambda with a generic capture list returning `MyTypeT`

_ZZ4mainENK3$_2clEv main::$_2::operator()() const
<0x3355e000> = distinct !DISubprogram(name: "operator()", linkageName: "_ZZ4mainENK3$_2clEv", scope: <0x3350c2d8>, file: <0x324ec940>, line: 133, type: <0x32ec9ab0>, scopeLine: 133, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x32514848>, declaration: <0x3355df80>, retainedNodes: <0x32692020>)
test-program.cpp
<0x334e98c0> = !DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeT", file: <0x324ec940>, line: 100, baseType: <0x334e9840>)
<0x32ec9a40> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x32c56ac0>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)


// capture-by-reference lambda taking a `float*` arugment returning `int`

_ZZ4mainENK3$_3clEPf main::$_3::operator()(float*) const
<0x33260ad0> = distinct !DISubprogram(name: "operator()", linkageName: "_ZZ4mainENK3$_3clEPf", scope: <0x33122f78>, file: <0x324ec940>, line: 129, type: <0x33263080>, scopeLine: 129, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x32514848>, declaration: <0x33260a50>, retainedNodes: <0x32692020>)
test-program.cpp
<0x325f4d48> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
<0x332609c0> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x33263010>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
<0x3355e070> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x32510c48>, size: 64)
```   

##### Polymorphic lambdas

Consider:

```c++
  auto auto_lambda = [](auto x) {
    return x * 2;
  };

  float autofloat = auto_lambda(3.14f);
  int autoint = auto_lambda(static_cast<int>(12));
```

This creates `template<class T> S operator()(T)` that gets instantiated twice: once for `int` and once for `float`. (`S` is the type of the result of the multiplication of `T` by an `int`)

As of the last commit of the `llvm-project` submodule, the mangled name of this language construct - `_ZZ4mainENK3$_0clIiEEDaT_` and `_ZZ4mainENK3$_0clIfEEDaT_` cannot be demangled via `llvm::Demangle`. This is also true for `GNU c++filt (GNU Binutils) 2.43.1`. Apparently the last part of the name `DaT_` stands for return type dedcuction with a placeholder for the returned type (which is available in the IR metadata). Resources for this:
* [warning! "sort-of-WTF" information ahead](https://quuxplusone.github.io/blog/2019/08/08/why-default-order-failed/)
* [llvm-project/clang/lib/AST/ItaniumMangle.cpp](https://github.com/llvm/llvm-project/blob/2c0b888359c6c5976054bb423ba1d7b37bae9f1a/clang/lib/AST/ItaniumMangle.cpp#L4558)
    * `::= Da # auto`
    * can't support for this be added to the `llvm::Demangle`?
    * digging further in [a dedicated document](./0x-llvm-demangling.md)

`types` metadata:

```
_ZZ4mainENK3$_0clIfEEDaT_ _ZZ4mainENK3$_0clIfEEDaT_
<0x4842e150> = distinct !DISubprogram(name: "operator()<float>", linkageName: "_ZZ4mainENK3$_0clIfEEDaT_", scope: <0x4888a5e8>, file: <0x47866940>, line: 146, type: <0x4842e000>, scopeLine: 146, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x4788e8d8>, templateParams: <0x488e2078>, declaration: <0x485db9f0>, retainedNodes: <0x47a0b4a0>)
// Types:
<0x4788ac48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
<0x4842df90> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x485da540>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
<0x4788ac48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)


_ZZ4mainENK3$_0clIiEEDaT_ _ZZ4mainENK3$_0clIiEEDaT_
<0x4842f910> = distinct !DISubprogram(name: "operator()<int>", linkageName: "_ZZ4mainENK3$_0clIiEEDaT_", scope: <0x4888a5e8>, file: <0x47866940>, line: 146, type: <0x4842e420>, scopeLine: 146, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: <0x4788e8d8>, templateParams: <0x4842e3d8>, declaration: <0x4842f880>, retainedNodes: <0x47a0b4a0>)
// Types:
<0x4796edd8> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
<0x4842df90> = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: <0x485da540>, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
<0x4796edd8> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
```

**Observations**

* (off-topic?) file `checksum` provided in the `DIFile` metadata - could be useful 
* reference type is wrapped in `!DIDerivedType(tag: DW_TAG_reference_type, ...` with the obvious `baseType`
    * pointer type is wrapped in a `DIDerivedType` with `DW_TAG_pointer_type`
* type alias (using/typedef) is wrapped in `!DIDerivedType(tag: DW_TAG_typedef, name: "MyTypeT", ... ` with the `baseType` referencing the aliased type (`!DIBasicType` of `float` or other alias - nested `using` - `!DIDerivedType`), suggesting a clear walk algorithm to get to the bottom-most type to determine the viability of the parameter capture

* `void` metadata return type is shown as `null` member of the `types` metadata tuple
e.g.
```
// void foo(int, float)
<0xe0d7760> = !DISubroutineType(types: <0xd6cb5b8>)
  <0xd6cb5b8> = !{null, <0xd389b08>, <0xd2a5c48>}
    <0xd389b08> = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
    <0xd2a5c48> = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
```
* metadata of `C` functions has the same structure
* arguments of a lambda are delimited by 2 `type` metadata values (return value and the pointer to the lambda object), i.e. lambda argument list begins after `DIFlagArtificial | DIFlagObjectPointer` type metadata object


## Custom LLVM IR Metadata

The following `llvm-project` diff allows "custom" metadata emission (unsafe first version)

```diff
diff --git a/clang/lib/CodeGen/CodeGenFunction.cpp b/clang/lib/CodeGen/CodeGenFunction.cpp
index dcf523f56bf..397a5b229b9 100644
--- a/clang/lib/CodeGen/CodeGenFunction.cpp
+++ b/clang/lib/CodeGen/CodeGenFunction.cpp
@@ -31,6 +31,7 @@
 #include "clang/AST/StmtObjC.h"
 #include "clang/Basic/Builtins.h"
 #include "clang/Basic/CodeGenOptions.h"
+#include "clang/Basic/SourceManager.h"
 #include "clang/Basic/TargetBuiltins.h"
 #include "clang/Basic/TargetInfo.h"
 #include "clang/CodeGen/CGFunctionInfo.h"
@@ -1117,6 +1118,15 @@ void CodeGenFunction::StartFunction(GlobalDecl GD, QualType RetTy,
         getLLVMContext(), VScaleRange->first, VScaleRange->second));
   }
 
+  if (FD && (getLangOpts().CPlusPlus || getLangOpts().C11 || getLangOpts().C17 || getLangOpts().C23 || getLangOpts().C2y || getLangOpts().C99)) {
+    auto &SourceManager = FD->getASTContext().getSourceManager();
+    bool InMainFile = SourceManager.isInMainFile(
+        SourceManager.getExpansionLoc(FD->getBeginLoc()));
+
+    Fn->addMetadata(InMainFile ? "VSTR_LOCAL" : "VSTR_INCLUDED", *llvm::MDNode::get(Fn->getContext(),
+    llvm::MDString::get(Fn->getContext(), "dummy_value")));
+  }
+
   llvm::BasicBlock *EntryBB = createBasicBlock("entry", CurFn);
 
   // Create a marker to make it easy to insert allocas into the entryblock
```
`FD` is `FunctionDecl`, a familiar type from the AST-modification attemtps.
`Fn` is an `llvm::Function*`.

I chose to create different metadata keys based on whether the `FunctionDecl` "is in main file" - I copied code from `isExpansionInMainFile` AST Matcher macro.

The relevant LLVM IR this modification produced on `test-program.cpp`:

```
!1597 = !{!"dummy_value"}

... SNIP ...

!dbg !1801 !VSTR_LOCAL !1597 {
  call void @hook_start(ptr 

... SNIP ...

unnamed_addr #3 comdat align 2 personality ptr @__gxx_personality_v0 !dbg !1810 !VSTR_INCLUDED !1597 {
```

Pre-meeting version (pre-meeting 14 Apr 2025):

We can see a instrumented function and a non-instrumented one.

* metadata (`!VSTR-NOT-SYSTEM-HEADER !6`) injected by AST plugin
  * `!6 = !{!"notinsystemheader"}`
  * defined in [a header file](../sandbox/01-llvm-ir/custom-metadata-pass/ast-meta-add/llvm-metadata.h)
* calls to library `call void @hook_*` injected by LLVM pass
  * the pass also created global `_ZZN2CX12NestedStruct10pubNestBarEfENKUlfE_clEfstring` strings

```
@_ZZN2CX12NestedStruct10pubNestBarEfENKUlfE_clEfstring = private unnamed_addr constant [78 x i8] c"CX::NestedStruct::pubNestBar(float)::'lambda'(float)::operator()(float) const\00", align 1

; Function Attrs: mustprogress noinline nounwind optnone uwtable
define linkonce_odr dso_local noundef i32 @_ZZN2CX12NestedStruct10pubNestBarEfENKUlfE_clEf(ptr noundef nonnull align 1 dereferenceable(1) %0, float noundef %1) #0 comdat align 2 !VSTR-NOT-SYSTEM-HEADER !6 {
  call void @hook_start(ptr @_ZZN2CX12NestedStruct10pubNestBarEfENKUlfE_clEfstring)
  call void @hook_float(float %1)
  %3 = alloca ptr, align 8
  %4 = alloca float, align 4
  store ptr %0, ptr %3, align 8
  store float %1, ptr %4, align 4
  %5 = load ptr, ptr %3, align 8
  %6 = load float, ptr %4, align 4
  %7 = fptosi float %6 to i32
  %8 = xor i32 %7, 123456789
  ret i32 %8
}

; Function Attrs: mustprogress noinline nounwind optnone uwtable
define linkonce_odr dso_local void @_ZNSt15__new_allocatorIcED2Ev(ptr noundef nonnull align 1 dereferenceable(1) %0) unnamed_addr #0 comdat align 2 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  ret void
}

!6 = !{!"notinsystemheader"}
```

# Snippets

## Metadata Dump

```c++
// Assumes Function& F in scope
if (auto* subprogram = F.getSubprogram(); subprogram) {
    subprogram->dump();
    outs() << subprogram->getFilename() << '\n';
    // change the "overload1" part to any filtering predicate (beware that some trees get extremely large)
    if (auto* type = subprogram->getType(); subprogram->getName() == "overload1" && type) {
        type->dumpTree();
    }
}
```

