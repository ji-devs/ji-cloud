use std::net::{SocketAddr, TcpListener};

/// Creates a [`TcpListener`] via `listenfd` if possible.
///
/// [`TcpListener`]: https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html
pub fn get_tcp_fd() -> Option<TcpListener> {
    #[cfg(feature = "listenfd")]
    let fd = listenfd::ListenFd::from_env().take_tcp_listener(0).unwrap();

    #[cfg(not(feature = "listenfd"))]
    let fd = None;

    fd
}

/// Get the port to run the server on.
pub fn get_addr(default: u16) -> SocketAddr {
    let port = std::env::var("PORT").map_or(default, |it| it.parse().unwrap_or(default));

    ([0, 0, 0, 0], port).into()
}
