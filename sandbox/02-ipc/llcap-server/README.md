# `llcap-server` - experimental test driver

## Usage:

    Usage: llcap-server [OPTIONS] --modmap <MODMAP> <COMMAND>

    Commands:
      trace-calls   Set up a function tracing server
      capture-args  
      help          Print this message or the help of the given subcommand(s)

    Options:
      -m, --modmap <MODMAP>        Sets path to module map directory as produced by the instrumentation
      -p, --fd-prefix <FD_PREFIX>  File descriptor prefixes for resources [default: /llcap-]
          --cleanup                Perform a cleanup of all possibly leftover resources related to the given stage and exit
      -v, --verbose...             Enable verbose output, write up to 3x
      -f, --full                   Perform the full tool iteration from the specified stage
      -a, --all-artifacts          Produce all artifacts that can be exported (in default locations) This option overrides ALL paths specified as out_file, or in_file for ANY stage
      -h, --help                   Print help
      -V, --version                Print version

### Function tracing mode

    Set up a function tracing server

    Usage: llcap-server --modmap <MODMAP> trace-calls [OPTIONS]

    Options:
      -c, --buff-count <BUFF_COUNT>  Buffer count [default: 10]
      -s, --buff-size <BUFF_SIZE>    Buffer size in bytes [default: 4194304]
      -o, --out-file <OUT_FILE>      output file, where a [TODO format] output will be stored [default: ./trace.out.txt]
      -h, --help                     Print help