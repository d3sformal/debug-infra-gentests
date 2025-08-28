#!/bin/bash

function run-test {
  TestDir=$1; shift;
  OutTestScript="$(pwd)"/"$1"; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  LlcapBufSz=$1; shift
  LlcapBufCnt=$1; shift
  ExtraArgs=$*; shift;

  IRTestScript="$OutTestScript"/ir;
  cd ../
  
  ./llcap-cleanup.sh
  ./test.sh ./"$TestDir" "$TestedFnName" "$Timeout" "$OutTestScript" "$IRTestScript" "$LlcapBufSz" "$LlcapBufCnt" $ExtraArgs

  if [[ "$?" != "0" ]]
  then
    echo "Failed test $TestDir with test scripts $OutTestScript and $IRTestScript with cmd ./test.sh ./$TestDir $TestedFnName $Timeout"
    cd ./test
    echo "Cmd:"
    echo "./test.sh ./"$TestDir" "$TestedFnName" "$Timeout" "$OutTestScript" "$IRTestScript" "$LlcapBufSz" "$LlcapBufCnt" \"\" \"$ExtraArgs\""
    exit 1
  fi
  cd ./test
  return 0
}

function run-test-in-directory-custom-buffers {
  TestDir=$1; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  LlcapBufSz=$1; shift
  LlcapBufCnt=$1; shift
  run-test "$TestDir" "../$TestDir/cases" "$TestedFnName" "$Timeout" "$LlcapBufSz" "$LlcapBufCnt"
}


function run-test-in-directory-fn-end-instr {
  TestDir=$1; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  LlcapBufSz=$1; shift
  LlcapBufCnt=$1; shift
  
  run-test "$TestDir" "../$TestDir/cases" "$TestedFnName" "$Timeout" "$LlcapBufSz" "$LlcapBufCnt" -mllvm -llcap-instrument-fn-exit
}

function run-tests-with-buffers {

  Size=$1; shift
  Count=$1; shift

  # testbin-* are directories where tests are run
  # the numeric literal is the test timeout
  run-test "testbin-arg-replacement-simple" "timeout-all.sh" "test_target" 0 "$Size" "$Count"
  
  # test_target - the name of the tested function (see the sources of the tests)
  run-test-in-directory-custom-buffers "testbin-arg-replacement-large" "test_target" 5 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-simple" "test_target" 5 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-structret-simple" "test_target" "2" "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-thisptr" "test_target" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-lambda-thisptr" "operator()" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-sret" "test_target" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-sret-structarg" "test_target" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-sret-this" "test_target" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-arg-replacement-sret-this-structarg" "test_target" 2 "$Size" "$Count"
  run-test-in-directory-custom-buffers "testbin-c-example" "test_target" 5 "$Size" "$Count"
  run-test "testbin-c-example" "timeout-all.sh" "test_target" 0 "$Size" "$Count"
  run-test-in-directory-fn-end-instr "testbin-arg-replacement-unc-exc" "test_target" 5 "$Size" "$Count"
}

# the defaults of the llcap-server
DefaultLlcapBufSz="4194304" 
DefaultLlcapBufCnt="10"

run-tests-with-buffers $DefaultLlcapBufSz $DefaultLlcapBufCnt

echo "Forcing small buffers and 2-buffer recycling"

# 8 is the minimum buffer size

run-tests-with-buffers 8 2

echo "Forcing small buffers and buffer recycling on a single buffer"

run-tests-with-buffers 8 1

echo "All done"
