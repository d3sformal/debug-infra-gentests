# End-to-end testing environment

End-to-end testing is implemented here. The `test.sh` script builds specified binary and traces it completely. The testing part is not yet implemented (planned: basic pattern matching on testing stage output).

It requires the following arguments: `<tested binary directory> <substring of a function name> <timeout in seconds> "" <additional clang arguments>`. (notice the `""`).

The test script performs tracing of all functions but tests only one of them, the very first one in the selection list `llcap-server` provides. Thus, for now, the tested binary shall ensure that it calls the desired function the most times out of the entire binary.

If any part of the test fails or is terminated unexpectedly and `llcap-server` is terminated forcefully, the `llcap-cleanup.sh` performs the necessary cleanup.

For each test, artifacts are stored in the `out` and `build` directories.


## Example

`./test.sh ./testbin-arg-replacement-simple/ test_target 2 ""`

`./test.sh testbin-arg-replacement-unc-exc/ test_target 4 "" -mllvm -llcap-instrument-fn-exit`

## Running "all" tests

```bash
cd ./test 
./run-all-tests.sh
```

This runs the `test/timeout-all.sh` check on the output of one of the instrumentation and testing  of `testbin-arg-replacement-simple`.

In each `testbin-*` directory, there shoudl be `cases` folder, which contains similarly-written check scripts (named `tc-*.sh`). These receive "clean" test output and check it. Depending on scripts' exit code, the entire test fails/continues.
