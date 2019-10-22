use actix_web::{web, Error, HttpResponse};
use futures::Future;
use juniper::graphiql::graphiql_source;
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::{Executor, FieldResult, FieldError};
use mongodb::coll::options::{IndexModel, IndexOptions};
use mongodb::oid::ObjectId;
use mongodb::{bson, doc, Client, ThreadedClient};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use wither::model::Model;
use wither_derive::Model;

use juniper_from_schema::graphql_schema_from_file;
graphql_schema_from_file!("src/schema.graphql");

pub struct Context {
    // wire up db here
    db_client: Client,
}
impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;

#[derive(Model, Serialize, Deserialize)]
pub struct Coffee {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[model(index(index = "asc", unique = "true"))]
    pub name: String,

    pub price: f64,

    pub image_url: String,

    pub description: Option<String>,
}



impl CoffeeFields for Coffee {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.as_ref().unwrap().to_hex()))
    }
    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
    fn field_price(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
        Ok(&self.price)
    }
    fn field_image_url(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.image_url)
    }
    fn field_description(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.description)
    }
}

/*
pub struct CoffeeInput {
    name: String,
    price: f64,
    description: String,
}
*/

// Query resolvers
impl QueryFields for Query {
    fn field_coffees(
        &self,
        _context: &Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Coffee>> {
        let mut result = Vec::new();

        let coffee = Coffee {
            id: Some(ObjectId::new().unwrap()),
            name: String::from("My coffee"),
            price: 0.5,
            image_url: String::from("images/espresso.jpg"),
            description: Some(String::from("Hello")),
        };

        result.push(coffee);
        Ok(result)
    }

    fn field_coffee(
        &self,
        _context: &juniper::Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
        _id: juniper::ID,
    ) -> FieldResult<Option<Coffee>> {
        let result = Coffee {
            id: Some(ObjectId::new().unwrap()),
            name: String::from("My coffee"),
            price: 0.5,
            image_url: String::from("images/espresso.jpg"),
            description: Some(String::from("Hello")),
        };
        Ok(Some(result))
    }
}

// Mutation resolvers
impl MutationFields for Mutation {
    fn field_create_coffee(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Coffee, Walked>,
        name: String,
        price: f64,
        description: Option<String>,
    ) -> FieldResult<Coffee> {
        let mut new_coffee = Coffee {
            id: Some(ObjectId::new().unwrap()),
            name,
            price,
            image_url: String::from(""),
            description,
        };

        // 1. Get context
        let context = executor.context();
        // 2. Get the db Connection
        let connection: Client = context.db_client.clone();
        // 3. Get the db
        let database = connection.db("coffeed");
        // 3. Get the collection
        // let collection: Collection = connection.db("coffed").collection("coffees");
        // 4. Create indexes
        // let index_model: IndexModel = IndexModel::new(keys: doc! {"name"}, options: Default);
        // collection.create_index_model(model: IndexModel);
        // 5. Write
        // let doc = doc! {
        //     "_id": new_coffee.id.to_string(),
        //     "name": new_coffee.name.to_string(),
        //     "price": new_coffee.price,
        //     "imageUrl": new_coffee.image_url.clone(),
        //     "description": new_coffee.description.clone().unwrap_or_else(|| String::from(""))
        // };
        // collection.insert_one(doc, None).unwrap();
        // Ok(new_coffee)
        // Err(error) => Err("Could not insert")
        // 4. Save
        new_coffee.save(database, None).unwrap();
        Ok(new_coffee)
    }
}

fn playground() -> HttpResponse {
    let html = playground_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    db_client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ctx = Context {
        db_client: db_client.get_ref().clone(),
    };

    web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/graphql", web::post().to_async(graphql))
        .route("/playground", web::get().to(playground))
        .route("/graphiql", web::get().to(graphiql));
}
