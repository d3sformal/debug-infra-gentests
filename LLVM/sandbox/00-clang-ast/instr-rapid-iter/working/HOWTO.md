# AST modification demo

* assumes `llvm-project` to be built, [`../../setup-tool.sh`](../../setup-tool.sh) executed at least once (to build, execute `ninja` in [sandbox/build](../../../build/))

* both demos use the [tracing library skeleton](../../inject-w-library/lib/)

## `program-instr.sh`

* modifies source code to integrate with tracing library
* outputs `modified-files.txt`, `fn-ids.csv`
* also compiles the modified source into `a.out`

By running `a.out`, you obtain (mostly) outputs from the tracing library (serialization data).

## `ftrace-program-instr.sh`

Same outputs as above, with the only difference when running `a.out`: an additional `log.txt` file that contains the traced function calls (ids correspond tho the ids in `fn-ids.csv`)
