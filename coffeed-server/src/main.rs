use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, App, HttpServer};
use mongodb::coll::options::IndexOptions;
use mongodb::coll::Collection;
use mongodb::{bson, db::ThreadedDatabase, doc, Client, ThreadedClient};
use pretty_env_logger;

pub mod routes;
pub mod schema;

// pub type MongoPool = r2d2::Pool<MongodbConnectionManager>;
// pub type MongoConnection = r2d2::PooledConnection<MongodbConnectionManager>;

fn create_db_client() -> Client {
    let client = Client::connect("localhost", 27017).expect("Failed to initialize client.");
    // Authenticate
    client
        .db("admin")
        .auth("username", "password")
        .expect("Could not authenticate.");

    client
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("ADDRESS", "127.0.0.1");

    pretty_env_logger::init();

    let port: u16 = 8082;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    let db_client = create_db_client();
    // Create indexes
    let collection: Collection = db_client.db("coffeed").collection("coffees");
    let mut name_index = IndexOptions::new();
    name_index.unique = Some(true);
    collection
        .create_index(doc! {"name": 1}, Some(name_index))
        .expect("Could not create index");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new())
            .wrap(middleware::Logger::default())
            // Save db_client in Server's state
            .data(db_client.clone())
            .configure(schema::register)
            // Serve images
            .service(actix_files::Files::new("/public", "src/public").show_files_listing())
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
