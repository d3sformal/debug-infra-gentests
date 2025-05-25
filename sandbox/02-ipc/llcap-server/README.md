# `llcap-server` - experimental test driver

## Build

The build depends on a "shared memory metadata struct" that can be found in the hook library's `shm_commons.h` file. If this file's location does not match (relatively) to this folder, you need to modify the (`header` call in the) `build.rs` file. 

## Usage:

    Usage: llcap-server [OPTIONS] --modmap <MODMAP> <COMMAND>

    Commands:
      trace-calls   Set up a function tracing server
      capture-args  
      help          Print this message or the help of the given subcommand(s)

    Options:
      -m, --modmap <MODMAP>          Sets path to module map directory as produced by the instrumentation
      -p, --fd-prefix <FD_PREFIX>    File descriptor prefixes for resources [default: /llcap-]
      -c, --buff-count <BUFF_COUNT>  Buffer count [default: 10]
      -s, --buff-size <BUFF_SIZE>    Buffer size in bytes [default: 4194304]
      -v, --verbose...               Enable verbose output, write up to 3x
      -f, --full                     Perform the full tool iteration starting from the selected stage
      -a, --all-artifacts            Produce all artifacts that can be exported (in default locations) This option overrides ALL paths specified as import or export paths for ANY stage
          --cleanup                  Perform a cleanup of all possibly leftover resources related to the given stage and exit
      -h, --help                     Print help
      -V, --version                  Print version

### Function tracing mode

    Set up a function tracing server

    Usage: llcap-server --modmap <MODMAP> trace-calls [OPTIONS]

    Options:
      -o, --out-file <OUT_FILE>        produce an export file [default: ./trace.out]
      -i, --import-path <IMPORT_PATH>  imports an export file, export file will not be created
      -h, --help                       Print help

### Argument capture mode (heavy WIP)

    Usage: llcap-server --modmap <MODMAP> capture-args [OPTIONS]

    Options:
      -s, --selection-file <SELECTION_FILE>
              [default: ./selected-fns.bin]
      -o, --out-dir <OUT_DIR>
              the directory where function argument traces are saved (or offloaded) [default: ./capture-out/]
      -l, --mem-limit <MEM_LIMIT>
              capture memory limit in MEBIBYTES - offloading will be performed to the output directory [default: 0]
      -h, --help
              Print help