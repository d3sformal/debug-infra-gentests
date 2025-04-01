# Notes on LLVM IR metadata

## Aims

Map the possibilities of LLVM IR metadata, specifically:

1. explore IR **metadata** capabilities wrt. **any function** (as a single entity)
2. explore IR **metadata** capabilities wrt. **a function's parameters** (e.g. tying metadata to specific function's arguments, all arguments, accross C++'s template instantiations, etc.)
3. discuss/decide **feasability** of exporting custom metadata from pre-IR-generating LLVM/Clang passes into the IR
4. the minimum outcome is deeper understanding of the LLVM architecture, LLVM IR syntax and semantics

## Concrete Why-s

* *LLVM pass, function "classification":* in clang's AST Matchers, we can query whether a function is built-in / library /`#include`d, in LLVM IR, this information is not available (not in a stable way)

* *LLVM pass, function argument capture:* exporting specific type information into the IR would allow the LLVM IR pass and accompanying library to support more types, easing extensions of the envisioned tools

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
