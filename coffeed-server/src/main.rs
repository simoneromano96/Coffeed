use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use pretty_env_logger;
extern crate r2d2;
extern crate r2d2_mongodb;

use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, VerifyPeer};

pub mod schema;

pub type MongoPool = r2d2::Pool<MongodbConnectionManager>;
pub type MongoConnection = r2d2::PooledConnection<MongodbConnectionManager>;

fn create_db_pool() -> MongoPool {
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            // .with_ssl(
            //     Some("path/to/ca.crt"),
            //     "path/to/client.crt",
            //     "path/to/client.key",
            //     VerifyPeer::Yes
            // )
            // .with_unauthenticated_ssl(
            //     Some("path/to/ca.crt"),
            //     VerifyPeer::No
            // )
            .with_db("admin")
            .with_auth("username", "password")
            .build(),
    );

    Pool::builder().build(manager).unwrap()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("ADDRESS", "127.0.0.1");

    pretty_env_logger::init();

    let port: u16 = 8080;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    let db_pool = create_db_pool();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new())
            .wrap(middleware::Logger::default())
            // Save db_client in Server's state
            .data(db_pool.clone())
            .configure(schema::register)
            // Serve images
            .service(actix_files::Files::new("/public", "src/public").show_files_listing())
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
