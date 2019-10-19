use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use pretty_env_logger;
extern crate mongodb;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson, Client, ThreadedClient};

pub mod schema;

fn create_db_client() -> Client {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let database = client.db("system");
    database.auth("username", "password").unwrap();

    client
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let port: u16 = 8080;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new())
            .wrap(middleware::Logger::default())
            .configure(schema::register)
            // Serve images
            .service(actix_files::Files::new("/public", "src/public").show_files_listing())
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
