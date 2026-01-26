# End-to-end demo

Assumes
* built `keepassxc` at least once
* built the repo (`llvm-project`, `llvm-pass`, `ipc-hooklib`)

**assuming pwd is this file's parent**
**assuming keepassxc is at the relative path ../../../../keepassxc**

## How to run (cli version with example captured data)

```shell
./e2e-cli.sh ./tmp-tracing ../../../../keepassxc
```

## How to run (GUI available)

In this directory:

```shell
mkdir -p ./tmp-tracing && ./make-diffs.sh ./tmp-tracing ../../../../keepassxc

rm -rf ./tmp-tracing/modmaps

./cli-calltrace.sh ./tmp-tracing ../../../../keepassxc

./cli-test.sh ./tmp-tracing ../../../../keepassxc
```


make sure to click the password generator and play around with it
* note: too much input data will cause the testing phase to take too long

```shell
mkdir ./tmp-tracing && ./make-diffs.sh ./tmp-tracing ../../../../keepassxc

./gui-calltrace.sh ./tmp-tracing ../../../../keepassxc

./gui-capture-test.sh ./tmp-tracing ../../../../keepassxc
```

### Other useful commands

* cleanup: `./manual-flush.sh`

* when keepass complains that only one instance can be running:
  
  killall -r ".\*keepassxc.\*" -s KILL

* removing `keepassxc/build` and rebuilding

