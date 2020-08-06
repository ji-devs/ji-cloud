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
    let mut port = default;

    match std::env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    ([0, 0, 0, 0], port).into()
}
