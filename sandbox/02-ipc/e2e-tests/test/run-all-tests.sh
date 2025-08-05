#!/bin/bash

function run-test {
  TestDir=$1; shift;
  OutTestScript="$(pwd)"/"$1"; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  ExtraArgs=$*; shift;

  IRTestScript="$OutTestScript"/ir;
  cd ../
  
  ./llcap-cleanup.sh
  ./test.sh ./"$TestDir" "$TestedFnName" "$Timeout" "$OutTestScript" "$IRTestScript" $ExtraArgs

  if [[ "$?" != "0" ]]
  then
    echo "Failed test $TestDir with test scripts $OutTestScript and $IRTestScript with cmd ./test.sh ./$TestDir $TestedFnName $Timeout"
    cd ./test
    echo "Cmd:"
    echo "./test.sh ./"$TestDir" "$TestedFnName" "$Timeout" "$OutTestScript" "$IRTestScript" \"\" \"$ExtraArgs\""
    exit 1
  fi
  cd ./test
  return 0
}

function run-test-in-directory {
  TestDir=$1; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  
  run-test "$TestDir" "../$TestDir/cases" "$TestedFnName" "$Timeout"
}

function run-test-in-directory-fn-end-instr {
  TestDir=$1; shift;
  TestedFnName=$1;shift;
  Timeout=$1; shift;
  
  run-test "$TestDir" "../$TestDir/cases" "$TestedFnName" "$Timeout" -mllvm -llcap-instrument-fn-exit
}

run-test "testbin-arg-replacement-simple" "timeout-all.sh" "test_target" "0"
run-test-in-directory "testbin-arg-replacement-simple" "test_target" "5"
run-test-in-directory "testbin-arg-replacement-structret-simple" "test_target" "2"
run-test-in-directory "testbin-arg-replacement-thisptr" "test_target" "2"
run-test-in-directory "testbin-arg-replacement-lambda-thisptr" "operator()" "2"
run-test-in-directory "testbin-arg-replacement-sret" "test_target" "2"
run-test-in-directory "testbin-arg-replacement-sret-structarg" "test_target" "2"
run-test-in-directory "testbin-arg-replacement-sret-this" "test_target" "2"
run-test-in-directory "testbin-arg-replacement-sret-this-structarg" "test_target" "2"
run-test-in-directory "testbin-c-example" "test_target" "5"
run-test "testbin-c-example" "timeout-all.sh" "test_target" "0"
run-test-in-directory-fn-end-instr "testbin-arg-replacement-unc-exc" "test_target" "5"

echo "All done"