# TODO:

* ~~prepare examples for function tracing prototype~~
* ~~prepare examples for parameter capture prototype~~
* ~~think about non-deterministic traces~~
    * ~~do we permit relying on function call determinism?~~
* ~~think about C++ objects - capturing inside of them, this, ...~~
* better source organisation, READMEs where possible
* ~~move `llvm-project` somewhere more sensible~~
* add links to commits/READMEs/other files for every "DONE" item in this file
* ~~unify SOLVED vs DONE items~~
* ~~public repo?~~
* ~~remove ZMQ references entirely (too expensive to maintain)~~
    * ~~ensure ALL links to ZMQ functionality actually link to a revision where ZMQ is available~~
        * ~~ensure the revision is the same, where possible~~
    * also ensure legacy examples work (test-pass-working)
* more diagrams
* ~~split 00-testing-* to analysis and progress updates~~
    * try link each other?
* ~~move `Capturing function arguments` somewhere else... and document the status quo~~
* establish terminology used *eveywhere* (but initial documents?)
    * test driver === `llcap-server`
    * test coordinator === the parent of all forked test cases
    * argument packet of a function `foo` === n-tuple of `foo`s arguments
    * argument capture of a function `foo` === (process of obtaining of the **or** the) set of all recorded argument packets of a function `foo`
    * hooklib, hook library, ...
    * ... ?
* add some tests for `llcap-server` and hooklib now that everything seems to be more or less stable? [in progress]
* ~~proper automatic cleanup / overwrite policies for outputs [in progress - especially in arg capture]~~
* try out various scenarios (timeouts of test cases, crashes) 
* add a simple argument replacement example
* add a guide for adding custom type support
* try compilation from scratch in a new environment 
* (optionally?) terminate program right after tested function returns
    * exception handling???
* investigate debug metadata usage instead of current approach
* validate argument splitting works/is disallowed
* update `llcap-server` readme

**List of topics:**

* [Data Capture Library](#topic-data-capture-library)
* [C++ support](#topic-c-support)
* [LLVM IR approach](#topic-llvm-ir-approach)
* [Typed value capture](#topic-typed-value-capture)
* [Testing / Isolation](#topic-testing--isolation)
* [Papers](#topic-papers)
* [Modification of incoming arguments](#topic-modification-of-incoming-arguments)
* [Extras](#topic-extras)

# TOPIC: Data Capture Library

* thoroughly check for alignment issues and other UB

## Capturing funciton arguments

* **[DONE]** how to treat **templated** code
    * let `X` be the set of types allowed for recording:
    * `template <class T>`, parameter of type `T`, where all instantiations of `T` in `X` - should be OK with naive source modification
    * not all instantiations have `T` in `X` -> compilation errors
        * how to resolve?
        * can clang provide per-instantiation decisionmaking? how will that affect compile times/feasability?

* **[DONE]** how to treat type aliases?
    * should be doable via clang's APIs (the underlying type should be resolved, right?)

## Capturing execution trace 

* use other tools?

* **[DONE]** C vs C++ function scope tracking
    * for now I only focus on C++ code, this method should allow tracing of even code which throws exceptions
    * C has no "reasonably non-colliding" namespaces
    * on the other hand: no annoying exceptions

* **[DONE]** use of C++'s `auto` keyword
    * limits C heavily (`auto` not used widely)
    * intricacies of C++'s type system & reference semantics
        * ensuring that modified source code makes exactly the same side effects as the unmodified one

# TOPIC: C++ support

*  **[DUPLICATE]** ~~methods of objects with AST modification (abandoned?)~~

# TOPIC: LLVM IR approach

* shows more promise than AST modification
* trouble with filtering (non) library funcitons - information not available in the IR
    * **[DONE]** **Idea**: add metadata to the functions in the IR that could tell the LLVM pass if the function is `#include`d, library, ...
        * [LLVM Discussion](https://discourse.llvm.org/t/how-to-distinguish-between-user-defined-function-in-a-program-and-library-functions/54401/7)
    * more metadata could help with a GUI integration later (emitting line informatio metadata - *using debug information metadata*, ...)
    * **[DONE]** inject custom metadata regarding (non)library/builtin functions
* **[DONE]** capturing inside a lambda, overall lambda instrumentation
* **[DONE]** LLVM [intrinsics](http://llvm.org/docs/LangRef.html#intrinsics) - can they be used?
    * **warning, can affect code generation**
    * lack of documentation (e.g. `llvm.memcpy`)
    * even then, seem inconsequential

* **[DONE]** **important for custom IR metadata**: `isExpansionInMainFile` vs `isExpansionInSystemHeader`
    * Does `isExpansionInMainFile` exclude functions in **any** header files?
        * If yes, does `isExpansionInSystemHeader` solve this?

* **[DONE]** is the custom metadata approach outlined in [](./01-llvm-ir-metadata-emission.md#custom-llvm-ir-metadata) correct?
* **[DONE]** is it possible to inject metadata in the AST Matching phase?
    * e.g. visit every `FunctionDecl` and add data (key-value string pairs) that would later be injected as LLVM IR metadata? (this is a question because the AST itself (and `FunctionDecl`) seemed to be "`const`ant" throughout my experimentation)

* **[DONE]** investigate [Clang IR (CIR)](https://llvm.github.io/clangir/)
    * relatively new feature (doesn't seem to be used with my build of LLVM) - *ClangIR upstreaming RFC was accepted in Feb 2024,*
    * conclusion: probably useful in far future (dialect augmentation)

* **[DONE]** add an option to metadata export plugin to use mangled name approach via a plugin option
    * supply LLVM pass & use `-mllvm -llcap-filter-by-mangled` when compiling

* `custom-metadata-pass` - document build steps

# TOPIC: Typed value capture
* **[DONE]** document hurdles / bypasses / alternatives to LLVM metadata approach
    * `this` pointer
    * ABIs that return structures by value in registers (`sret`, `structureReturn`, `StructRet`)
    * **?maybe?** - dive deeper into the ABI specs and cause runtime error within the LLVM plugin if the target ABI is unknown/unhandled
* **[DONE]** document current approach
* **[DONE]** discuss possible breakage points
    * structures fitting 64 bits passed in registers
    * changes in argument order
* try to use LLVM debug information metadata to perform instrumentation **without** the LLVM metadata customization  

# TOPIC: Testing / Isolation
* isolation inspiration - to investigate? - (`clone`/`fork` used): [mem-isolate](https://github.com/brannondorsey/mem-isolate)
* document architecture of testing, new hooklib, diagrams, ...
* send termination signal to a fork that timed out
* `I | [main] Run program for fn m: F7DB0979 f: 03000000` - add name and module path (nonverbose output)

# TOPIC: Papers

* investigate memory instruction instrumentation via Pin from the AURORA tool
    * [repo](https://github.com/RUB-SysSec/aurora)

# TOPIC: Modification of incoming arguments
* **[DONE?]** are there assumptions in the IR, that would break if we replaced arguments mid-function?
    * **[DONE]** the idea is to "introduce new `alloca`d variable (`%x`)" for each `%n` argument and replace each `%n` occurence with the new `%x`
* `const`ness vs `IR`-level modifications
    * does `-O3`, etc. impact the number of arguments in IR? (e.g. via more agressive const propagation?)
    * if prohibitive for argument "hijacking", can we at least "remove" `const`
* **[DONE]** validate approach that modifies the IR
* add smaller demo (replace the current demo) that shows everything on a smaller scale
    * particularly the argument replacement part (simple visual confirmation for the user)
* document the architecture, add diagrams

# TOPIC: Extras
* **[CLOSED]** report mangling bug (after rebuild of latest llvm project)
    * actually a [larger issue](https://discourse.llvm.org/t/rfc-clang-diagnostic-for-demangling-failures/82835) ([pull request for a diagnostic](https://github.com/llvm/llvm-project/pull/111391))

* so far only on x64 linux ABI - try more?
* attachment to external tools
    * Valgrind, GDB - obvious candidates (instrument to conditionally trigger a breakpoint?)
    * property-checking tools (unsinged over/underflow, no-progress loops, suspicious iteration counts, recording & analysis of comparisons at particular location)
    * GCOV - coverage
* aggregate mode of the tools - run test cases with multiple failing instances to allow cross-checking (by the user)
* custom "log expressions" that would report into the framework? (additional info) 
    * ! compatibility with LLVM IR
```c++
    RET_T foo(PT p1, PT p2) { 

        /* start of static instrumentation */
        
        /* user-provided expression that can be serialized/logged 
           e.g. p1 * p2 / (p1 - p2) / ... */
        /* end of static instrumentation */
        
        // function body...
    }
```


# Revamp of buffer reading and writng

Currently the language rules around mutability and sharing when reading and writing shared
memory buffers are enforced by `RefCell` and a check that "no raw pointer is ever casted from
`*const` to `*mut`".

The following snipped is a prototype of a checked and fully encapsulated "reader" from
a pointer (to shared memory). It provides a restrictive interface that disallow the above-mentioned
cast.

```rust
/// an object that represents reading permission on a given contiguous section of
/// memory
pub struct ByteReader<'a> {
  borrow: Ref<'a, *const u8>,
  /// the absolute limit of the contiguous section
  raw_end: *const u8,
  offset: usize
  /// a custom limit of the contiguous section
  limit: usize
  // the combination of offset + limit allows to work within buffer chunks of the single
  // large shared memory block
}

impl<'a> ByteReader<'a> {
  /// Safety: caller must ensure that `borrowed_start` points to a delimited contiguous
  /// block of memory ending at `raw_end`
  pub unsafe fn new(borrowed_start: Ref<'a, *const u8>, raw_end: *const u8) -> Self {
    Self {
        borrow: borrowed_start,
        raw_end,
        offset: 0,
        limit: 0
    }
  }

  /// sets the offset for all subsequent read operations
  pub fn set_offset(&mut self, offset: usize) -> Result<()> {
    let _ = ptr_add_nowrap(*self.borrow, offset)?;
    self.offset = offset;
    Ok(())
  }

  /// sets the limit for all subsequent read operations
  pub fn set_limit(&mut self, limit: usize) -> Result<()> {
    ensure!(limit < self,offset, "Invalid limit - vs offset");
    ensure!((*self.raw_end < (ptr_add_nowrap(*self.borrow, limit)?)), "Invalid limit - vs raw_end");
    self.limit = limit;
    Ok(())
  }

  /// reads n bytes
  pub fn read_n(&self, nbytes: usize) -> Result<Vec<u8>> {
    let ptr = ptr_add_nowrap(*self.borrow, self.offset)?;
    let limit_end = ptr_add_nowrap(*self.borrow, self.limit)?;
   
    overread_check(ptr, self.raw_end, nbytes, "bytereader read_n end")?;
    overread_check(ptr, limit_end, nbytes, "bytereader read_n limit")?;
   
    ensure!(!ptr.is_null(), "read_n null ptr");
    ensure!((nbytes as isize) < isize::MAX, "read_n nbytes too large");
    // SAFETY: check above + the `Self::new` guarantees about the pointer + Ref + alignment of a byte is always satisfied
    let slice = unsafe { std::slice::from_raw_parts(ptr, nbytes) }; 
    Ok(Vec::from(slice))
  }

  /// safety: T is "reasonable" (rule of thumb: primitive types)
  /// reads T at the specified offset
  pub unsafe fn read<T: Sized + Copy>(&self) -> Result<T> {
    let ptr = ptr_add_nowrap(*self.borrow, self.offset)?;
    let limit_end = ptr_add_nowrap(*self.borrow, self.limit)?;
    let sz = std::mem::size_of::<T>();

    overread_check(ptr, self.raw_end, sz, "bytereader read end")?;
    overread_check(ptr, limit_end, sz, "bytereader read limit")?;

    ensure!(!ptr.is_null(), "read_n null ptr");
    // SAFETY: reasonable T
    return Ok(unsafe { read_w_alignment_chk::<T>(ptr) }?)
  }
}
```

After I semi-completed this prototype, I realized that too much refactoring would be done
threatening hours of debugging in case of oversight, thus I'm shelving the idea here. 
To complete the abstraction, a **writer** interface would also be needed.