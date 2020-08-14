use std::net::{SocketAddr, TcpListener};

#[cfg(feature = "local")]
pub fn get_tcp_fd() -> Option<TcpListener> {
    listenfd::ListenFd::from_env().take_tcp_listener(0).unwrap()
}

#[cfg(not(feature = "local"))]
pub fn get_tcp_fd() -> Option<TcpListener> {
    // we don't have listenfd here.
    None
}

pub fn get_addr(default: u16) -> SocketAddr {
    let port = std::env::var("PORT").map_or(default, |it| it.parse().unwrap_or(default));

    ([0, 0, 0, 0], port).into()
}
