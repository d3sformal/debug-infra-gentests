# Intro, resources

* [2023 LLVM Dev Mtg - MLIR Is Not an ML Compiler, and Other Common Misconceptions](https://www.youtube.com/watch?v=lXAp6ZAWyBY)
    * [slides](https://llvm.org/devmtg/2023-10/slides/techtalks/Zinenko-MLIRisNotAnMLCompiler.pdf)
* [2023 EuroLLVM - MLIR Dialect Design and Composition for Front-End Compilers](https://www.youtube.com/watch?v=hIt6J1_E21c)
* [MLIR Tutorial: Building a Compiler with MLIR](https://users.cs.utah.edu/~mhall/mlir4hpc/pienaar-MLIR-Tutorial.pdf)

## MLIR dialects

* intermixed inside MLIR
* no clear dialect to go from and to
* has 2 built-in operations
    * and 10 types
    * and upstream dialects
        * "standard" dialect - broken down to 11 dialects
* we can apply transformations - passes - to MLIR, later lowered to LLVM
    * lowering passes - MUST occur
    * test passes
    * optimization passes
* interfaces on operations
    * allow bufferization, other optimizations
    * allow lowering to LLVM/CUDA

### MLIR compiler components

**The importer**
- simplifies importing data (from bytecode, a graph sctructure, ...)

**The funnel**
- funneling inputs into a common representation

**The legalizer** -> **The optimizer**

**The specializer**
- e.g. CUDA version dialects, ...

**The exporter**
- e.g. the LLVM

Tensorflow example:

![Tensorflow MLIR compiler (src MLIR Dialect Design @ 7:38)](tensorflow-mlir.png)

Fortran Example:

![Fortran example (src MLIR Dialect Design @ 11:16)](fortran-mlir.png)

CIRCT Example (a !hardware!-design platform)

![CIRCT](CIRCT-MLIR.png)

- continue 11:30


### Dialects

* a building block
* namespaces for attribtues, operations and types
* dialect `hooks` and `interfaces` inform MLIR how to thread dialect's operations
* library API design <=> dialect design
* can consist of other dialects (typically sharing typesystem)
* some dialects can dissapear (be used only for e.g. optimization passes)

**Characteristics**
* contracts over attribtues, operations and types
* tradeoffs!
* important to understand the context (how the dialect interacts with other dialects)
* types are more important than operations (harder to evolve them)



