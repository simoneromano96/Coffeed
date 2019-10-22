use actix_web::{web, Error, HttpResponse};
use futures::Future;
use juniper::graphiql::graphiql_source;
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::{graphql_value, Executor, FieldError, FieldResult};
use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::oid::ObjectId;
use mongodb::{bson, doc, Client, ThreadedClient};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

use juniper_from_schema::graphql_schema_from_file;
graphql_schema_from_file!("src/schema.graphql");

pub struct Context {
    db_client: Client,
}
impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;

#[derive(Serialize, Deserialize, Clone)]
pub struct Coffee {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub description: Option<String>,
}

impl CoffeeFields for Coffee {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_hex()))
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
    description: Option<String>,
}

impl CoffeeInputFields for CoffeeInput {
    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
    fn field_price(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
        Ok(&self.price)
    }
    // fn field_image_url(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
    //     Ok(&self.image_url)
    // }
    fn field_description(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.description)
    }
}
*/

// Query resolvers
impl QueryFields for Query {
    fn field_coffees(
        &self,
        executor: &Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Coffee>> {
        // 1. Get context
        let context = executor.context();
        // 2. Get the db Connection
        let connection: Client = context.db_client.clone();
        // 3. Get the db
        let database = connection.db("coffeed");
        // 4. Get collection
        let collection: Collection = database.collection("coffees");
        // 6. Find coffees
        let coffees = collection.find(None, None).expect("Document not found");
        // 7. Deserialize the document into a Coffee instance
        let mut result: Vec<Coffee> = Vec::new();
        for coffee_document in coffees {
            if let Ok(item) = coffee_document {
                let coffee: Coffee = bson::from_bson(bson::Bson::Document(item))?;
                result.push(coffee);
            }
        }
        Ok(result)
    }

    fn field_coffee(
        &self,
        executor: &juniper::Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
        id: juniper::ID,
    ) -> FieldResult<Coffee> {
        // 1. Get context
        let context = executor.context();
        // 2. Get the db Connection
        let connection: Client = context.db_client.clone();
        // 3. Get the db
        let database = connection.db("coffeed");
        // 4. Get collection
        let collection: Collection = database.collection("coffees");
        // 5. Convert objectId
        let oid = ObjectId::with_string(&id).expect("Id not valid");
        // 6. Find coffee
        let result_document = collection
            .find_one(Some(doc! { "_id":  oid }), None)?
            .expect("Document not found");
        // 7. Deserialize the document into a Coffee instance
        let result: Coffee = bson::from_bson(bson::Bson::Document(result_document))?;
        Ok(result)
    }
}

// Mutation resolvers
impl MutationFields for Mutation {
    // TODO Handle error!
    fn field_create_coffee(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Coffee, Walked>,
        data: CoffeeInput,
    ) -> FieldResult<Coffee> {
        let new_coffee = Coffee {
            id: ObjectId::new().unwrap(),
            name: data.name,
            price: data.price,
            image_url: String::from(""),
            description: data.description,
        };

        // 1. Get context
        let context = executor.context();
        // 2. Get the db Connection
        let connection: Client = context.db_client.clone();
        // 3. Get the db
        let database = connection.db("coffeed");
        // 4. Get collection
        let collection: Collection = database.collection("coffees");
        // 5. Serialize
        let bson = bson::to_bson(&new_coffee)?;
        // 6. Save
        if let bson::Bson::Document(document) = bson {
            collection.insert_one(document, None)?; // Insert into a MongoDB collection
        }

        Ok(new_coffee)
    }
}

fn playground() -> HttpResponse {
    let html = playground_source("http://127.0.0.1:8082/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8082/graphql");
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