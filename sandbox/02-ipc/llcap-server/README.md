# `llcap-server` - experimental test driver

## Usage:

    Usage: llcap-server [OPTIONS] --modmap <FILE> <COMMAND>

    Commands:
      zero-mq  Use (failed experimental) ZeroMQ capture
      shmem    
      help     Print this message or the help of the given subcommand(s)

    Options:
      -m, --modmap <FILE>  Sets path to module map directory as produced by the instrumentation
      -v, --verbose...     Enable verbose output, write up to 3x
      -h, --help           Print help
      -V, --version        Print version

### (failed) Zero-MQ mode

    Usage: llcap-server --modmap <FILE> zero-mq [OPTIONS]

    Options:
      -s, --socket <SOCKET>  Socket address to operate on [default: ipc:///tmp/zmq-socket]
      -h, --help             Print help

### Shared-memory mode

    Usage: llcap-server --modmap <FILE> shmem [OPTIONS]

    Options:
      -f, --fd-prefix <FD_PREFIX>    File descriptor prefixes for resources [default: /llcap-]
      -c, --buff-count <BUFF_COUNT>  Buffer count [default: 4]
      -s, --buff-size <BUFF_SIZE>    Buffer size in bytes [default: 4096]
      -h, --help                     Print help