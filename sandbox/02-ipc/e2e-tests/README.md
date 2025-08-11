# End-to-end testing environment

End-to-end testing is implemented here. The `test.sh` script builds specified binary and traces it completely. The testing part is not yet implemented (planned: basic pattern matching on testing stage output).

It requires the following arguments: `<tested binary directory> <substring of a function name> <timeout in seconds> "" <additional clang arguments>`. (notice the `""`).

The test script performs tracing of all functions but tests only one of them, the very first one in the selection list `llcap-server` provides. Thus, for now, the tested binary shall ensure that it calls the desired function the most times out of the entire binary.

If any part of the test fails or is terminated unexpectedly and `llcap-server` is terminated forcefully, the `llcap-cleanup.sh` performs the necessary cleanup.

For each test, artifacts are stored in the `out` and `build` directories.

## Running "all" tests

```bash
cd ./test 
./run-all-tests.sh
```

## How it works

`run-all-tests.sh` refers to test directories that provide small C/C++ applications that are going to be tested. The line

 run-test "testbin-arg-replacement-simple" "timeout-all.sh" "test_target" "0"

This runs the `test/timeout-all.sh` check on the output of one of the instrumentation and testing of `testbin-arg-replacement-simple`.

The line

 run-test-in-directory "testbin-arg-replacement-simple" "test_target" "5"

Describes a test that is set up and performed based on the contents of the `testbin-arg-replacement-simple` directory. Most test cases follow this structure, providing the directory, name of the function to test, and timeout (in seconds).

In each `testbin-*` directory, the scripts look for the `cases` folder, which contains check scripts (named `tc-*.sh`). These receive "clean" test output and check it. 

The "clean" test output is created by reducing the `llcap-server` output - stripping white spaces and unnecessary lines, ... See the (bottom of the) [`test.sh` file for implementation](./test.sh) for details. For a reference check script, head [here](./testbin-arg-replacement-structret-simple/cases/tc-first-call.sh) and compare it with [`llcap-server` outputs](../example-arg-replacement/README.md#primitive-type-instrumentation). It is expected that the check script will fail to indicate a test failure. 

Optionally, the (call-tracing) instrumentation can be checked by inspecting the LLVM IR (folder `cases/ir`, scripts `ir-*.sh`). Again, depending on the script's exit code, the entire test fails or testing continues.
