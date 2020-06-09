mod routes;
mod cors;

use std::net::SocketAddr;
use std::env;
use cfg_if::cfg_if;
use routes::get_routes;
use crate::settings::SETTINGS;

use warp:: {
    http::{
        Method,
    },
    Reply,
    Rejection,
    Filter,
};

cfg_if! {
    if #[cfg(feature = "local")] {
        use listenfd::ListenFd;
        use std::convert::Infallible;
        use warp:: {
            hyper:: {
                self,
                Server,
                Body, 
                Request, 
                Response
            },
        };

        //auto reload on code change
        //see: https://github.com/seanmonstar/warp/blob/master/examples/autoreload.rs
        pub async fn start_server() {
            // hyper let's us build a server from a TcpListener (which will be
            // useful shortly). Thus, we'll need to convert our `warp::Filter` into
            // a `hyper::service::MakeService` for use with a `hyper::server::Server`.
            let make_svc = hyper::service::make_service_fn(|_: _| async { Ok::<_, Infallible>(
                warp::service(get_routes())
            ) });

            let mut listenfd = ListenFd::from_env();
            // if listenfd doesn't take a TcpListener (i.e. we're not running via
            // the command above), we fall back to explicitly binding to a given
            // host:port.
            let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
                Server::from_tcp(l).unwrap()
            } else {
                Server::bind(&get_addr())
            };

            server.serve(make_svc).await.unwrap();
        }
    } else { 
        pub async fn start_server() {
            warp::serve(get_routes())
                .run(get_addr())
                .await;
        }
    }
}

fn get_addr() -> SocketAddr {

    let mut port = SETTINGS.get().unwrap().port;

    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {port = n;},
                Err(_e) => {},
            };
        }
        Err(_e) => {},
    };
    
    ([0, 0, 0, 0], port).into()
}
