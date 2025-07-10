# End-to-end testing environment

End-to-end testing is implemented here. The `test.sh` script builds specified binary and traces it completely. The testing part is not yet implemented (planned: basic pattern matching on testing stage output).

It requires the following arguments: `<tested binary directory> <index of the function> <timeout in seconds>`.

The test script performs tracing of all functions but tests only one of them, the very first one in the selection list `llcap-server` provides. Thus, for now, the tested binary shall ensure that it calls the desired function the most times out of the entire binary.

If any part of the test fails or is terminated unexpectedly and `llcap-server` is terminated forcefully, the `llcap-cleanup.sh` performs the necessary cleanup.

For each test, artifacts are stored in the 

## Example

`./test.sh ./testbin-arg-replacement-simple/ 0 2`

