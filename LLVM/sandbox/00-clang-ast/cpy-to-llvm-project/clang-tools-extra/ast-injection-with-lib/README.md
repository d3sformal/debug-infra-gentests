# WIP version of an ASTMather-based clang tool

* we tried to rewrite the program source code so that tracing library functions are called at specific points
    * baseline: function **entry/exit** tracking (entry necessary) & simple **parameter capture**

The tool is only supposed to run on `C++` sources. Function execution tracing is done via a "scope guard" provided by the library that we expect exists. Parameter tracing is done via simple function call injection (to functions of a library that we expect to exist - see [`inject-w-library/lib`](../../../inject-w-library/lib/)).

You can get a rough idea about how the program is modified by looking into the [`include/fragments.hpp`](./include/fragments.hpp) file. 

The tool is unfinished and will probably remain so due to the issues outlined in the sources and because we decided that LLVM IR is a more suitable point for instrumentation insertion later.

By default, the tool performs function execution instrumentation.

## Options of the compiled tool

`-T` - Switches the instrumentation, the tool inserts parameter recording instrumentation

`-M name` - Output a list of modified files into a file "`name`"

`-I name` - Output function id mapping into a file "`name`"
