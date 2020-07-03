//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod options;

use diesel::{
    pg::PgConnection,
    Connection
};
use dotenv::dotenv;
use ji_cloud_shared::backend::google::{get_secret, get_access_token_and_project_id};
use simplelog::*;
use diesel_migrations::embed_migrations;
use options::{Opts, Target, get_target};
use structopt::StructOpt;

embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let target = get_target(&opts.target);

    dotenv().ok();

    init_logger();

    let db_connection_string = {
        if target == Target::Sandbox || target == Target::Release {
            let credentials_env_var = match target {
                Target::Local => "NONE",
                Target::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
                Target::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
            };
            let (token, project_id) = get_access_token_and_project_id(credentials_env_var).await.expect("couldn't get access token and project id!");

            let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
            let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;

            format!("postgres://postgres:{}@localhost:6432/jicloud", db_pass)
        } else {
            let db_user = std::env::var("LOCAL_DB_USER").expect("When not using Cloud Sql Proxy, set LOCAL_DB_USER in .env");
            let db_pass = std::env::var("LOCAL_DB_PASS").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PASS in .env");
            let db_port = std::env::var("LOCAL_DB_PORT").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PORT in .env");
            let db_name = std::env::var("LOCAL_DB_NAME").expect("When not using Cloud Sql Proxy, set LOCAL_DB_NAME in .env");
            format!("postgres://{}:{}@localhost:{}/{}", db_user, db_pass, db_port, db_name)
        }
    };

    if(opts.connection_string_only) {
        println!("\nConnection string:\n{}\n", db_connection_string);
    } else {
        let db_connection = PgConnection::establish(&db_connection_string).expect(&format!("Error connecting to database"));
        
        if(opts.verbose) {
            embedded_migrations::run_with_output(&db_connection, &mut std::io::stdout());
        } else {
            embedded_migrations::run(&db_connection);
        }
    }
}


fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
    ])
    .unwrap();
}
