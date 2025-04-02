# WIP tracking library & sample application

* you may ignore the sample application (`main.cpp`)

## Tracking Library

* `ASTMatcher`-based tool(s) ([sibling folder](../cpy-to-llvm-project/)) modify source code to utilize the types and functions available in this library to trace functions

This approach presents an annoying problem: *either* the header(s) of this library must be `#include`d in the modified source files (hence why the [`ast-injection-with-lib`](../cpy-to-llvm-project/clang-tools-extra/ast-injection-with-lib/) tool outputs the list of modified files) *or* the function prototypes as well as types have to be copied into the modified sources (essentially doing the `#include` work of the preprocessor manually). In the current version, `#include`s are prepended to modified files via [`lib/add_includes.sh`](./lib/add_includes.sh) script.

For an example that puts all the parts together, see the [`instr-rapid-iter/working/ftrace-program-instr.sh`](../instr-rapid-iter/working/ftrace-program-instr.sh) script.

