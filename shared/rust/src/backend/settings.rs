use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DbTarget {
    Local,
    Proxy,
    Remote(RemoteTarget),
}

#[derive(Debug)]
pub struct DbCredentials {
    pub dbname:String,
    pub user:String,
    pub pass:String,
    pub endpoint: DbEndpoint,
}
#[derive(Debug)]
pub enum DbEndpoint {
    Socket(String),
    Tcp(String, u16)
}

impl DbCredentials {
    pub fn new(secret_db_pass:&str, db_target:DbTarget) -> Self {
        match db_target {
            DbTarget::Local => {
                //these are env vars since it depends on developer's local machine
                let user = std::env::var("LOCAL_DB_USER").expect("When not using Cloud Sql Proxy, set LOCAL_DB_USER in .env");
                let pass = std::env::var("LOCAL_DB_PASS").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PASS in .env");
                let port = std::env::var("LOCAL_DB_PORT").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PORT in .env");
                let dbname = std::env::var("LOCAL_DB_NAME").expect("When not using Cloud Sql Proxy, set LOCAL_DB_NAME in .env");
                let host = "localhost".to_string();

                let port:u16 = port.parse().expect("Port must be a u32");

                Self { user, pass, dbname, endpoint: DbEndpoint::Tcp(host, port) }
            },
            DbTarget::Proxy => {
                Self { 
                    user: REMOTE_DB_USER.to_string(), 
                    pass: secret_db_pass.to_string(),
                    dbname: REMOTE_DB_NAME.to_string(), 
                    endpoint: DbEndpoint::Tcp("localhost".to_string(), SQL_PROXY_PORT)
                }
            },
            DbTarget::Remote(remote_target) => {
                let instance_connection = 
                    std::env::var("INSTANCE_CONNECTION_NAME")
                        .unwrap_or(match remote_target {
                            RemoteTarget::Sandbox => DB_INSTANCE_SANDBOX.to_string(),
                            RemoteTarget::Release => DB_INSTANCE_RELEASE.to_string(), 
                            _ => panic!("non-dev mode only makes sense for sandbox or release")
                        });

                let socket_path = std::env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

                Self { 
                    user: REMOTE_DB_USER.to_string(), 
                    pass: secret_db_pass.to_string(),
                    dbname: REMOTE_DB_NAME.to_string(), 
                    endpoint: DbEndpoint::Socket(format!("{}/{}", socket_path, instance_connection))
                }
                /*
                let instance_connection = 
                    std::env::var("INSTANCE_CONNECTION_NAME")
                        .unwrap_or(match remote_target {
                            RemoteTarget::Sandbox => DB_INSTANCE_SANDBOX.to_string(),
                            RemoteTarget::Release => DB_INSTANCE_RELEASE.to_string(), 
                            _ => panic!("non-dev mode only makes sense for sandbox or release")
                        });

                let socket_path = std::env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

                let full_socket_path = utf8_percent_encode(&format!("{}/{}", socket_path, instance_connection), NON_ALPHANUMERIC).to_string();

                log::warn!("connection string is: postgres://{}:PASSWORD@{}/{}", REMOTE_DB_USER, full_socket_path, REMOTE_DB_NAME);
                let connection_string = format!("postgres://{}:{}@{}/{}", REMOTE_DB_USER, secret_db_pass, full_socket_path, REMOTE_DB_NAME);

                connection_string
                */
            }
        }
    }

    pub fn to_string(&self) -> String {
        match &self.endpoint {
            DbEndpoint::Tcp(host, port) => format!("postgres:///{}?user={}&password={}&host={}&port={}", self.dbname, self.user, self.pass, host, port),
            DbEndpoint::Socket(path) => format!("postgres:///{}?user={}&password={}&host={}", self.dbname, self.user, self.pass, path),
        }
    }
}
