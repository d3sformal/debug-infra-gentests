pub struct Constants {}

impl Constants {
    pub const fn default_socket_path() -> &'static str {
        "/tmp/zmq-socket"
    }

    pub const fn default_socket_address() -> &'static str {
        "ipc:///tmp/zmq-socket"
    }

    pub const fn parse_fnid_radix() -> u32 {
        10
    }
}
